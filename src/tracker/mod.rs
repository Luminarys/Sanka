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
use private::PrivateTracker;
use config::{TrackerConfig, PrivateConfig};

use std::sync::Arc;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::thread;
use time::SteadyTime;

pub struct Tracker {
    pub torrents: Mutex<HashMap<String, Torrent>>,
    pub stats: Mutex<Stats>,
    pub private: PrivateTracker,
    pub config: TrackerConfig,
}

impl Default for Tracker {
    fn default() -> Tracker {
        Tracker::new(Default::default(), Default::default())
    }
}

impl Tracker {
    pub fn new(config: TrackerConfig, pconfig: PrivateConfig) -> Tracker {
        let torrents: Mutex<HashMap<String, Torrent>> = Mutex::new(Default::default());
        let stats = Mutex::new(Stats::new());
        let private = PrivateTracker::new(pconfig);
        Tracker {
            torrents: torrents,
            stats: stats,
            private: private,
            config: config,
        }
    }

    pub fn start_updaters(tracker: Arc<Tracker>) {
        let tracker_reap = tracker.clone();
        thread::spawn(move || {
            info!("Starting reaper!");
            loop {
                thread::sleep(tracker_reap.config.reap_interval);
                tracker_reap.reap();
            }
        });

        if cfg!(feature = "private") {
            let tracker_priv_flush = tracker.clone();
            thread::spawn(move || {
                info!("Starting delta flusher!");
                loop {
                    thread::sleep(tracker_priv_flush.private.config.flush_interval);
                    tracker_priv_flush.private.flush();
                }
            });
            let tracker_priv_update = tracker.clone();
            thread::spawn(move || {
                info!("Starting private updater!");
                loop {
                    thread::sleep(tracker_priv_update.private.config.update_interval);
                    tracker_priv_update.private.update();
                }
            });
        }
    }

    pub fn handle_announce(&self, announce: Announce) -> Result<SuccessResponse, ErrorResponse> {
        let mut torrents = self.torrents.lock();
        let mut tracker_stats = self.stats.lock();
        let torrent = if torrents.contains_key(&announce.info_hash) {
            let t = torrents.get_mut(&announce.info_hash).unwrap();
            tracker_stats.announces += 1;
            tracker_stats.peers -= t.get_peer_count();
            let delta = t.update(&announce);
            if cfg!(feature = "private") {
                self.private.add_announce(delta);
            }
            tracker_stats.peers += t.get_peer_count();
            t
        } else {
            tracker_stats.torrents += 1;
            tracker_stats.announces += 1;
            tracker_stats.peers += 1;

            let t = Torrent::new(announce.info_hash.clone());
            torrents.insert(announce.info_hash.clone(), t);
            let t = torrents.get_mut(&announce.info_hash).unwrap();
            let delta = t.update(&announce);
            if cfg!(feature = "private") {
                self.private.add_announce(delta);
            }
            t
        };
        let peers = torrent.get_peers(announce.numwant, announce.action);
        let stats = torrent.get_stats();
        Ok(SuccessResponse::Announce(AnnounceResponse::new(peers,
                                                           stats,
                                                           announce.compact,
                                                           self.config.announce_interval,
                                                           self.config.min_announce_interval)))
    }

    pub fn handle_scrape(&self, scrape: Scrape) -> Result<SuccessResponse, ErrorResponse> {
        let mut torrents = HashMap::new();
        for hash in scrape.torrents {
            match self.torrents.lock().get(&hash) {
                Some(ref t) => {
                    let stats = t.get_stats();
                    torrents.insert(hash.clone(), stats);
                }
                None => {}
            };
        }

        let mut tracker_stats = self.stats.lock();
        tracker_stats.scrapes += 1;

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
        let to_del: Vec<_> = self.torrents.lock()
                                 .iter()
                                 .filter_map(|(k, torrent)| {
                                     if SteadyTime::now() - torrent.last_action >
                                        self.config.min_torrent_update_interval {
                                         Some(k.clone())
                                     } else {
                                         None
                                     }
                                 })
                                 .collect();
        for torrent in to_del {
            stats.torrents -= 1;
            self.torrents.lock().remove(&torrent);
        }

        let to_reap: Vec<_> = self.torrents.lock()
                                  .iter()
                                  .filter_map(|(k, torrent)| {
                                      if SteadyTime::now() - torrent.last_action >
                                         self.config.min_torrent_update_interval {
                                          None
                                      } else {
                                          Some(k.clone())
                                      }
                                  })
                                  .collect();
        stats.peers = 0;
        for info_hash in to_reap {
            match self.torrents.lock().get_mut(&info_hash) {
                Some(ref mut t) => {
                    t.reap(&self.config.min_peer_update_interval);
                    stats.peers += t.get_peer_count();
                }
                None => {}
            }
        }
    }
}
