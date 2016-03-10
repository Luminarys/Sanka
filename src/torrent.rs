use std::collections::HashMap;
use time::SteadyTime;

use announce::{Announce, Action};
use peer::{Peer, Delta};
use std::net::{SocketAddrV4, SocketAddrV6};

pub struct Torrent {
    hash: String,
    snatches: u64,
    seeders: HashMap<String, Peer>,
    leechers: HashMap<String, Peer>,
    last_action: SteadyTime,
}

impl Torrent {
    pub fn new(hash: String) -> Torrent {
        Torrent {
            hash: hash,
            snatches: 0,
            seeders: HashMap::new(),
            leechers: HashMap::new(),
            last_action: SteadyTime::now(),
        }
    }

    pub fn update(&mut self, a: Announce) -> Delta {
        match a.action {
            Action::Seeding => {
                if self.seeders.contains_key(&a.peer_id) {
                    match self.seeders.get_mut(&a.peer_id) {
                        Some(peer) => peer.update(&a),
                        None => Delta { peer_id: a.peer_id.clone(), upload: 0, download: 0, left: 0 }
                    }
                } else {
                    self.seeders.insert(a.peer_id.clone(), Peer::new(&a));
                    Delta { peer_id: a.peer_id.clone(), upload: 0, download: 0, left: 0 }
                }
            },
            Action::Leeching => {
                if self.leechers.contains_key(&a.peer_id) {
                    match self.leechers.get_mut(&a.peer_id) {
                        Some(peer) => peer.update(&a),
                        None => Delta { peer_id: a.peer_id.clone(), upload: 0, download: 0, left: 0 }
                    }
                } else {
                    self.leechers.insert(a.peer_id.clone(), Peer::new(&a));
                    Delta { peer_id: a.peer_id.clone(), upload: 0, download: 0, left: 0 }
                }
            },
            Action::Completed => {
                let mut peer = match self.leechers.remove(&a.peer_id) {
                    Some(p) => p,
                    None => Peer::new(&a)
                };
                let d = peer.update(&a);
                self.seeders.insert(a.peer_id.clone(), peer);
                d
            },
            Action::Stopped => {
                match (self.leechers.remove(&a.peer_id), self.seeders.remove(&a.peer_id)) {
                    (Some(ref mut peer), None) => peer.update(&a),
                    (_, Some(ref mut peer)) => peer.update(&a),
                    (None, None) => Delta { peer_id: a.peer_id.clone(), upload: 0, download: 0, left: 0 }
                }
            }
        }
    }

    pub fn get_peers(&self, amount: u8, action: Action) -> (Vec<u8>, Vec<u8>) {
        let mut peers = Vec::with_capacity(6 * amount as usize);
        let mut peers6 = Vec::with_capacity(18*amount as usize);
        match action {
            Action::Leeching => {
                let count = get_ips(&mut peers, &mut peers6, &self.seeders, amount);
                if count == amount {
                    (peers, peers6)
                } else {
                    get_ips(&mut peers, &mut peers6, &self.leechers, amount - count);
                    (peers, peers6)
                }
            },
            Action::Stopped => {
                (peers, peers6)
            },
            _ => {
                // Seeding or completed - prefer leechers
                let count = get_ips(&mut peers, &mut peers6, &self.leechers, amount);
                if count == amount {
                    (peers, peers6)
                } else {
                    get_ips(&mut peers, &mut peers6, &self.seeders, amount - count);
                    (peers, peers6)
                }
            }
        }
    }
}

fn get_ips(peers: &mut Vec<u8>, peers6: &mut Vec<u8>, peer_dict: &HashMap<String, Peer>, wanted: u8) -> u8 {
    let mut count = 0;
    for peer in peer_dict.values() {
        if count == wanted {
            break;
        }
        match (peer.ipv4, peer.ipv6) {
            (Some(v4), Some(v6)) => {
                peers.append(&mut v4_to_bytes(&v4));
                peers6.append(&mut v6_to_bytes(&v6));
                count += 1;
            },
            (Some(v4), None) => {
                peers.append(&mut v4_to_bytes(&v4));
                count += 1;
            }
            (None, Some(v6)) => {
                peers6.append(&mut v6_to_bytes(&v6));
                count += 1;
            },
            (None, None) => { }
        }
    }
    count
}

fn v4_to_bytes(s: &SocketAddrV4) -> Vec<u8> {
    let mut v = Vec::with_capacity(6);
    v.extend(s.ip().octets().iter().cloned());
    v.extend(u16_to_u8(s.port()).iter().cloned());
    v
}

fn v6_to_bytes(s: &SocketAddrV6) -> Vec<u8> {
    let mut v = Vec::with_capacity(18);
    for seg in s.ip().segments().iter() {
        v.extend(u16_to_u8(seg.clone()).iter().cloned());
    }
    v.extend(u16_to_u8(s.port()).iter().cloned());
    v
}

fn u16_to_u8 (i: u16) -> [u8; 2] {
    [(i >> 8) as u8, (i & 0xff) as u8]
}

