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

use std::sync::{Mutex, MutexGuard};
use std::collections::HashMap;
use time::SteadyTime;
use time::Duration;

pub struct Tracker {
    pub torrents: Mutex<HashMap<String, Torrent>>,
    pub stats: Mutex<Stats>,
    pub private: PrivateTracker,
}

impl Tracker {
    pub fn new() -> Tracker {
        let torrents: Mutex<HashMap<String, Torrent>> = Mutex::new(Default::default());
        let stats = Mutex::new(Stats::new());
        let private = PrivateTracker::new();
        Tracker {
            torrents: torrents,
            stats: stats,
            private: private
        }
    }

    fn unlock_stats(&self) -> MutexGuard<Stats> {
        match self.stats.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    fn unlock_torrents(&self) -> MutexGuard<HashMap<String, Torrent>> {
        match self.torrents.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    pub fn handle_announce(&self, announce: Announce) -> Result<SuccessResponse, ErrorResponse> {
        let mut torrents = self.unlock_torrents();
        let mut tracker_stats = self.unlock_stats();
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
        Ok(SuccessResponse::Announce(AnnounceResponse::new(torrent.get_peers(announce.numwant, announce.action), torrent.get_stats(), announce.compact)))
    }

    pub fn handle_scrape(&self, scrape: Scrape) -> Result<SuccessResponse, ErrorResponse> {
        let mut torrents = HashMap::new();
        for hash in scrape.torrents {
            match self.unlock_torrents().get(&hash) {
                Some(ref t) => {
                    let stats = t.get_stats();
                    torrents.insert(hash.clone(), stats);
                }
                None => {}
            };
        }

        let mut tracker_stats = self.unlock_stats();
        tracker_stats.scrapes += 1;

        Ok(SuccessResponse::Scrape(ScrapeResponse { torrents: torrents }))
    }

    pub fn get_stats(&self) -> Result<SuccessResponse, ErrorResponse> {
        let ref stats = *self.unlock_stats();
        let resp = StatsResponse::new(stats);
        Ok(SuccessResponse::Stats(resp))
    }

    pub fn reap(&self) {
        // Clear stats
        let mut stats = self.unlock_stats();
        stats.update();
        // Delete torrents which are too old, and reap peers for the others.
        let to_del: Vec<_> = self.unlock_torrents()
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
            stats.torrents -= 1;
            self.unlock_torrents().remove(&torrent);
        }

        let to_reap: Vec<_> = self.unlock_torrents()
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
        stats.peers = 0;
        for info_hash in to_reap {
            match self.unlock_torrents().get_mut(&info_hash) {
                Some(ref mut t) => {
                    t.reap();
                    stats.peers += t.get_peer_count();
                }
                None => {}
            }
        }
    }
}
