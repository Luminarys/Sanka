use std::net::{SocketAddrV4, SocketAddrV6};

use tracker::torrent::{Stats, Peers};

pub struct Announce {
    pub info_hash: String,
    pub peer_id: String,
    pub ipv4: Option<SocketAddrV4>,
    pub ipv6: Option<SocketAddrV6>,
    pub ul: u64,
    pub dl: u64,
    pub left: u64,
    pub action: Action,
    pub numwant: u8,
    pub compact: bool,
}

#[derive(Clone)]
pub enum Action {
    Seeding,
    Leeching,
    Completed,
    Stopped,
}

pub struct AnnouncePeer {
    pub id: String,
    pub ipv4: Option<SocketAddrV4>,
    pub ipv6: Option<SocketAddrV6>,
}

pub struct AnnounceResponse {
    peers: Peers,
    stats: Stats,
    compact: bool
}

impl AnnouncePeer {
    pub fn get_ipv4_bytes(&self) -> Option<Vec<u8>> {
        match self.ipv4 {
            None => None,
            Some(sock) => {
                let mut v = Vec::with_capacity(6);
                v.extend(sock.ip().octets().to_vec());
                v.extend(u16_to_u8(sock.port()));
                Some(v)
            }
        }
    }

    pub fn get_ipv4_str(&self) -> Option<String> {
        match self.ipv4 {
            None => None,
            Some(sock) => {
                Some(format!("{}", sock.ip()))
            }
        }
    }

    pub fn get_ipv6_bytes(&self) -> Option<Vec<u8>> {
        match self.ipv6 {
            None => None,
            Some(sock) => {
                let mut v = Vec::with_capacity(18);
                for seg in sock.ip().segments().iter() {
                    v.extend(u16_to_u8(seg.clone()));
                }
                v.extend(u16_to_u8(sock.port()));
                Some(v)
            }
        }
    }

    pub fn get_ipv6_str(&self) -> Option<String> {
        match self.ipv6 {
            None => None,
            Some(sock) => {
                Some(format!("{}", sock.ip()))
            }
        }
    }
}

fn u16_to_u8(i: u16) -> Vec<u8> {
    vec![(i >> 8) as u8, (i & 0xff) as u8]
}

impl AnnounceResponse {
    pub fn new(peers: Peers, stats: Stats, compact: bool) -> AnnounceResponse {
        AnnounceResponse {
            peers: peers,
            stats: stats,
            compact: compact,
        }
    }

    pub fn peers(&self) -> &Peers {
        &self.peers
    }

    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    pub fn compact(&self) -> bool {
        self.compact
    }
}
