use time;
use std;

pub struct TrackerConfig {
    pub reap_interval: std::time::Duration,
    pub announce_interval: std::time::Duration,
    pub min_announce_interval: std::time::Duration,
    pub min_torrent_update_interval: time::Duration,
    pub min_peer_update_interval: time::Duration,
}

pub struct PrivateConfig {
    pub flush_interval: std::time::Duration,
    pub update_interval: std::time::Duration,
}

pub struct HttpConfig {
    pub listen_addr: String
}

impl Default for TrackerConfig {
    fn default() -> TrackerConfig {
        TrackerConfig {
            reap_interval: std::time::Duration::from_secs(120),
            announce_interval: std::time::Duration::from_secs(1800),
            min_announce_interval: std::time::Duration::from_secs(900),
            min_torrent_update_interval: time::Duration::seconds(2000),
            min_peer_update_interval: time::Duration::seconds(2000),
        }
    }
}

impl Default for PrivateConfig {
    fn default() -> PrivateConfig {
        PrivateConfig {
            flush_interval: std::time::Duration::from_secs(5),
            update_interval: std::time::Duration::from_secs(900),
        }
    }
}

impl Default for HttpConfig {
    fn default() -> HttpConfig {
        HttpConfig {
            listen_addr: String::from("127.0.0.1:8000")
        }
    }
}
