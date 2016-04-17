use tracker::announce::Announce;

use time::SteadyTime;
use std::net::{SocketAddrV4, SocketAddrV6};

pub struct Peer {
    pub id: String,
    uploaded: u64,
    downloaded: u64,
    left: u64,
    pub last_action: SteadyTime,
    pub ipv4: Option<SocketAddrV4>,
    pub ipv6: Option<SocketAddrV6>,
}

pub struct Delta {
    pub peer_id: String,
    pub upload: u64,
    pub download: u64,
    pub left: u64,
}

impl Peer {
    pub fn new(a: &Announce) -> Peer {
        Peer {
            id: a.peer_id.clone(),
            uploaded: a.ul,
            downloaded: a.dl,
            left: a.left,
            ipv4: a.ipv4,
            ipv6: a.ipv6,
            last_action: SteadyTime::now(),
        }
    }

    pub fn update(&mut self, a: &Announce) -> Delta {
        let d = Delta {
            peer_id: self.id.clone(),
            upload: if a.ul > self.uploaded {
                a.ul - self.uploaded
            } else {
                0
            },
            download: if a.dl > self.downloaded {
                a.dl - self.downloaded
            } else {
                0
            },
            left: if self.left > a.left {
                self.left - a.left
            } else {
                0
            },
        };
        self.uploaded = a.ul;
        self.downloaded = a.dl;
        self.left = a.left;
        self.ipv4 = a.ipv4;
        self.ipv6 = a.ipv6;
        self.last_action = SteadyTime::now();
        d
    }

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

impl Delta {
    pub fn new(peer_id: String) -> Delta {
        Delta {
            peer_id: peer_id,
            upload: 0,
            download: 0,
            left: 0,
        }
    }
}
