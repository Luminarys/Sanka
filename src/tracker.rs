use hyper::server::Request;
use concurrent_hashmap::ConcHashMap;
use torrent::Torrent;
use std::collections::HashMap;
use error::ErrorResponse;
use announce::{AnnounceResponse, Announce, Action};
use scrape::ScrapeResponse;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use time::SteadyTime;
use time::Duration;

pub struct Tracker {
    pub torrents: ConcHashMap<String, Torrent>,
}

impl Tracker {
    pub fn handle_announce(&self,
                           req: &Request,
                           param_vec: Vec<(String, String)>)
                           -> Result<AnnounceResponse, ErrorResponse> {
        let mut params = HashMap::new();
        for (key, val) in param_vec {
            params.insert(key, val);
        }

        let info_hash: String = try!(get_from_params(&params, String::from("info_hash")));
        let pid = try!(get_from_params(&params, String::from("peer_id")));
        let ul = try!(get_from_params(&params, String::from("uploaded")));
        let dl = try!(get_from_params(&params, String::from("downloaded")));
        let left = try!(get_from_params(&params, String::from("left")));

        // IP parsing according to BEP 0007 with additional proxy forwarding check
        let port = try!(get_from_params(&params, String::from("port")));
        let default_ip = match req.headers.get_raw("X-Forwarded-For") {
            Some(bytes) => {
                match String::from_utf8(bytes[0].clone()) {
                    Ok(ip_str) => {
                        match ip_str.parse::<IpAddr>() {
                            Ok(ip) => SocketAddr::new(ip, port),
                            Err(_) => req.remote_addr,
                        }
                    }
                    Err(_) => req.remote_addr,
                }
            }
            None => req.remote_addr,
        };
        let ip = match get_from_params(&params, String::from("ip")) {
            Ok(ip) => SocketAddr::new(ip, port),
            Err(_) => default_ip,
        };

        let (ipv4, ipv6) = match ip {
            SocketAddr::V4(v4) => {
                let v6 = match get_socket(&params, String::from("ipv6"), port) {
                    Some(sock) => {
                        match sock {
                            SocketAddr::V6(v6) => Some(v6),
                            _ => None,
                        }
                    }
                    None => None,
                };
                (Some(v4), v6)
            }
            SocketAddr::V6(v6) => {
                let v4 = match get_socket(&params, String::from("ipv4"), port) {
                    Some(sock) => {
                        match sock {
                            SocketAddr::V4(v4) => Some(v4),
                            _ => None,
                        }
                    }
                    None => None,
                };
                (v4, Some(v6))
            }
        };

        let action = match get_from_params::<String>(&params, String::from("event")) {
            Ok(ev_str) => {
                match &ev_str[..] {
                    "started" => get_action(left),
                    "stopped" => Action::Stopped,
                    "completed" => Action::Completed,
                    _ => get_action(left),
                }
            }
            Err(_) => get_action(left),
        };

        let numwant = match get_from_params::<u8>(&params, String::from("numwant")) {
            Ok(amount) => {
                if amount > 25 {
                    25
                } else {
                    amount
                }
            }
            Err(_) => 25,
        };

        let announce = Announce {
            info_hash: info_hash.clone(),
            peer_id: pid,
            ipv4: ipv4,
            ipv6: ipv6,
            ul: ul,
            dl: dl,
            left: left,
            action: action.clone(),
            numwant: numwant,
        };

        let (_delta, stats, peers) = match self.torrents.find_mut(&info_hash) {
            Some(ref mut accessor) => {
                let mut t = accessor.get();
                let delta = t.update(announce);
                (delta, t.get_stats(), t.get_peers(numwant, action))
            }
            None => {
                let mut t = Torrent::new(info_hash.clone());
                let delta = t.update(announce);
                let resp = (delta, t.get_stats(), t.get_peers(numwant, action));
                self.torrents.insert(info_hash, t);
                resp
            }
        };
        Ok(AnnounceResponse {
            peers: peers,
            stats: stats,
        })
    }

    pub fn handle_scrape(&self,
                         params: Vec<(String, String)>)
                         -> Result<ScrapeResponse, ErrorResponse> {
        let mut torrents = HashMap::new();
        for (_, hash) in params {
            match self.torrents.find(&hash) {
                Some(ref accessor) => {
                    let t = accessor.get();
                    let stats = t.get_stats();
                    torrents.insert(hash.clone(), stats);
                }
                None => {}
            };
        }
        Ok(ScrapeResponse { torrents: torrents })
    }

    pub fn reap(&self) {
        // Delete torrents which are too old, and reap the others.
        let to_del: Vec<_> =
            self.torrents.iter()
            .filter_map(|(k, torrent)| {
                if SteadyTime::now() - torrent.last_action > Duration::seconds(3600) {
                    Some(k.clone())
                } else {
                    None
                }
            })
            .collect();
        for torrent in to_del {
            self.torrents.remove(&torrent);
        }

        let to_reap: Vec<_> =
            self.torrents.iter()
            .filter_map(|(k, torrent)| {
                if SteadyTime::now() - torrent.last_action > Duration::seconds(3600) {
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
                None => { }
            }
        }
    }
}

fn get_from_params<T: FromStr>(map: &HashMap<String, String>,
                               key: String)
                               -> Result<T, ErrorResponse> {
    match map.get(&key) {
        Some(res) => {
            match res.parse::<T>() {
                Ok(val) => Ok(val),
                Err(_) => Err(ErrorResponse::BadRequest),
            }
        }
        None => Err(ErrorResponse::BadRequest),
    }
}

fn get_socket(params: &HashMap<String, String>, key: String, port: u16) -> Option<SocketAddr> {
    let ip: Result<IpAddr, ErrorResponse> = get_from_params(params, key.clone());
    let socket: Result<SocketAddr, ErrorResponse> = get_from_params(params, key.clone());
    match (ip, socket) {
        (Err(_), Err(_)) => None,
        (Ok(ip), Err(_)) => Some(SocketAddr::new(ip, port)),
        (Err(_), Ok(sock)) => Some(sock),
        _ => None,
    }
}

fn get_action(left: u64) -> Action {
    if left == 0 {
        Action::Seeding
    } else {
        Action::Leeching
    }
}
