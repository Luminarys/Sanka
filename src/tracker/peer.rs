use tracker::announce::{Announce, AnnouncePeer};

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

    pub fn get_announce_peer(&self) -> AnnouncePeer {
        AnnouncePeer {
            id: self.id.clone(),
            ipv4: self.ipv4.clone(),
            ipv6: self.ipv6.clone(),
        }
    }
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
