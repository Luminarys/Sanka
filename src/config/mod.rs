use time;
use std;
use toml::{Table, Value};

#[derive(Default)]
pub struct MainConfig {
    pub tracker: TrackerConfig,
    pub private: PrivateConfig,
    pub http: HttpConfig,
}

impl MainConfig {
    pub fn from_toml(toml: Table) -> MainConfig {
        let tracker = toml.get("tracker")
            .map_or(None, |t| Some(TrackerConfig::from_toml(t)))
            .unwrap_or_default();

        let private = toml.get("private")
            .map_or(None, |t| Some(PrivateConfig::from_toml(t)))
            .unwrap_or_default();

        let http = toml.get("http")
            .map_or(None, |t| Some(HttpConfig::from_toml(t)))
            .unwrap_or_default();

        MainConfig {
            tracker: tracker,
            private: private,
            http: http,
        }
    }
}

#[derive(Clone)]
pub struct TrackerConfig {
    pub reap_interval: std::time::Duration,
    pub announce_interval: std::time::Duration,
    pub min_announce_interval: std::time::Duration,
    pub min_torrent_update_interval: time::Duration,
    pub min_peer_update_interval: time::Duration,
}

#[derive(Clone)]
pub struct PrivateConfig {
    pub flush_interval: std::time::Duration,
    pub update_interval: std::time::Duration,
    pub extra: Option<Table>,
}

#[derive(Clone)]
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

impl TrackerConfig {
    fn from_toml(toml: &Value) -> TrackerConfig {
        match *toml {
            Value::Table(ref t) => {
                let announce_interval = t.get("announce_interval")
                    .map_or(None, |v| v.as_integer())
                    .map_or(None, |v| Some(std::time::Duration::from_secs(v as u64)))
                    .unwrap_or(std::time::Duration::from_secs(1800));
                let reap_interval = t.get("reap_interval")
                    .map_or(None, |v| v.as_integer())
                    .map_or(None, |v| Some(std::time::Duration::from_secs(v as u64)))
                    .unwrap_or(std::time::Duration::from_secs(120));
                let min_announce_interval = t.get("min_announce_interval")
                    .map_or(None, |v| v.as_integer())
                    .map_or(None, |v| Some(std::time::Duration::from_secs(v as u64)))
                    .unwrap_or(std::time::Duration::from_secs(900));
                let min_torrent_update_interval = t.get("min_torrent_update_interval")
                    .map_or(None, |v| v.as_integer())
                    .map_or(None, |v| Some(time::Duration::seconds(v as i64)))
                    .unwrap_or(time::Duration::seconds(900));
                let min_peer_update_interval = t.get("min_peer_update_interval")
                    .map_or(None, |v| v.as_integer())
                    .map_or(None, |v| Some(time::Duration::seconds(v as i64)))
                    .unwrap_or(time::Duration::seconds(900));
                TrackerConfig {
                    reap_interval: reap_interval,
                    announce_interval: announce_interval,
                    min_announce_interval: min_announce_interval,
                    min_torrent_update_interval: min_torrent_update_interval,
                    min_peer_update_interval: min_peer_update_interval,
                }
            }
            _ => Default::default()
        }
    }
}

impl Default for PrivateConfig {
    fn default() -> PrivateConfig {
        PrivateConfig {
            flush_interval: std::time::Duration::from_secs(5),
            update_interval: std::time::Duration::from_secs(900),
            extra: None,
        }
    }
}

impl PrivateConfig {
    fn from_toml(toml: &Value) -> PrivateConfig {
        match *toml {
            Value::Table(ref t) => {
                let flush_interval = t.get("flush_interval")
                    .map_or(None, |v| v.as_integer())
                    .map_or(None, |v| Some(std::time::Duration::from_secs(v as u64)))
                    .unwrap_or(std::time::Duration::from_secs(5));
                let update_interval = t.get("update_interval")
                    .map_or(None, |v| v.as_integer())
                    .map_or(None, |v| Some(std::time::Duration::from_secs(v as u64)))
                    .unwrap_or(std::time::Duration::from_secs(900));
                PrivateConfig {
                    flush_interval: flush_interval,
                    update_interval: update_interval,
                    extra: Some(t.clone())
                }
            }
            _ => Default::default()
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

impl HttpConfig {
    fn from_toml(toml: &Value) -> HttpConfig {
        match *toml {
            Value::Table(ref t) => {
                let listen_addr = t.get("listen_addr")
                    .map_or(None, |v| v.as_str())
                    .map_or(None, |v| Some(String::from(v)))
                    .unwrap_or(String::from("127.0.0.1:8000"));
                HttpConfig {
                    listen_addr: listen_addr,
                }
            }
            _ => Default::default()
        }
    }
}
