use std::time::Duration;

pub struct TrackerConfig {
    reap_interval: Duration,
    min_announce_interval: Duration,
    announce_interval: Duration,
    min_torrent_update_interval: Duration,
    min_peer_update_interval: Duration,
}

pub struct PrivateConfig {
    flush_interval: Duration,
    update_interval: Duration,
}

pub struct HttpConfig {
    pub listen_addr: String
}

impl Default for TrackerConfig {
    fn default() -> TrackerConfig {
        TrackerConfig {
            reap_interval: Duration::from_secs(120),
            min_announce_interval: Duration::from_secs(900),
            announce_interval: Duration::from_secs(1800),
            min_torrent_update_interval: Duration::from_secs(2000),
            min_peer_update_interval: Duration::from_secs(2000),
        }
    }
}

impl Default for PrivateConfig {
    fn default() -> PrivateConfig {
        PrivateConfig {
            flush_interval: Duration::from_secs(5),
            update_interval: Duration::from_secs(900),
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
