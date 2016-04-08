pub mod torrent;
pub mod peer;
pub mod scrape;
pub mod announce;
pub mod stats;

use self::torrent::Torrent;
use self::announce::{AnnounceResponse, Announce};
use self::scrape::{ScrapeResponse, Scrape};
use self::stats::{Stats, StatsResponse};
use response::error::ErrorResponse;
use response::success::SuccessResponse;

use spin::Mutex;
use concurrent_hashmap::ConcHashMap;
use std::collections::HashMap;
use time::SteadyTime;
use time::Duration;

pub struct Tracker {
    pub torrents: ConcHashMap<String, Torrent>,
    pub stats: Mutex<Stats>,
}

impl Tracker {
    pub fn new() -> Tracker {
        let torrents: ConcHashMap<String, Torrent> = Default::default();
        let stats = Mutex::new(Stats::new());
        Tracker {
            torrents: torrents,
            stats: stats,
        }
    }

    pub fn handle_announce(&self, announce: Announce) -> Result<SuccessResponse, ErrorResponse> {
        let mut tracker_stats = self.stats.lock();
        tracker_stats.announces += 1;
        let (_delta, stats, peers) = match self.torrents.find_mut(&announce.info_hash) {
            Some(ref mut accessor) => {
                let mut t = accessor.get();
                let delta = t.update(&announce);
                (delta,
                 t.get_stats(),
                 t.get_peers(announce.numwant, announce.action))
            }
            None => {
                let mut t = Torrent::new(announce.info_hash.clone());
                let delta = t.update(&announce);
                let resp = (delta,
                            t.get_stats(),
                            t.get_peers(announce.numwant, announce.action));
                self.torrents.insert(announce.info_hash, t);
                resp
            }
        };

        Ok(SuccessResponse::Announce(AnnounceResponse {
            peers: peers,
            stats: stats,
        }))
    }

    pub fn handle_scrape(&self, scrape: Scrape) -> Result<SuccessResponse, ErrorResponse> {
        let mut tracker_stats = self.stats.lock();
        tracker_stats.scrapes += 1;
        let mut torrents = HashMap::new();
        for hash in scrape.torrents {
            match self.torrents.find(&hash) {
                Some(ref accessor) => {
                    let t = accessor.get();
                    let stats = t.get_stats();
                    torrents.insert(hash.clone(), stats);
                }
                None => {}
            };
        }
        Ok(SuccessResponse::Scrape(ScrapeResponse { torrents: torrents }))
    }

    pub fn get_stats(&self) -> Result<SuccessResponse, ErrorResponse> {
        let ref stats = *self.stats.lock();
        let resp = StatsResponse::new(stats);
        Ok(SuccessResponse::Stats(resp))
    }

    pub fn reap(&self) {
        // Clear stats
        let mut stats = self.stats.lock();
        stats.update();
        // Delete torrents which are too old, and reap peers for the others.
        let to_del: Vec<_> = self.torrents
                                 .iter()
                                 .filter_map(|(k, torrent)| {
                                     if SteadyTime::now() - torrent.last_action >
                                        Duration::seconds(3600) {
                                         Some(k.clone())
                                     } else {
                                         None
                                     }
                                 })
                                 .collect();
        for torrent in to_del {
            self.torrents.remove(&torrent);
        }

        let to_reap: Vec<_> = self.torrents
                                  .iter()
                                  .filter_map(|(k, torrent)| {
                                      if SteadyTime::now() - torrent.last_action >
                                         Duration::seconds(3600) {
                                          None
                                      } else {
                                          Some(k.clone())
                                      }
                                  })
                                  .collect();
        for info_hash in to_reap {
            match self.torrents.find_mut(&info_hash) {
                Some(ref mut accessor) => {
                    let mut t = accessor.get();
                    t.reap();
                }
                None => {}
            }
        }
    }
}
