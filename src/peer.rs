use time::SteadyTime;
use std::net::{SocketAddrV4, SocketAddrV6};
use announce::Announce;

pub struct Peer {
    id: String,
    uploaded: u64,
    downloaded: u64,
    left: u64,
    last_action: SteadyTime,
    pub ipv4: Option<SocketAddrV4>,
    pub ipv6: Option<SocketAddrV6>,
}

#[derive(Debug)]
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
            last_action: SteadyTime::now()
        }
    }

    pub fn update(&mut self, a: &Announce) -> Delta {
        let d = Delta {
            peer_id: self.id.clone(),
            upload: if a.ul > self.uploaded { a.ul - self.uploaded } else { 0 },
            download: if a.dl > self.downloaded { a.dl - self.downloaded } else { 0 },
            left: if self.left > a.left { self.left - a.left } else { 0 },
        };
        self.uploaded = a.ul;
        self.downloaded = a.dl;
        self.left = a.left;
        self.ipv4 = a.ipv4;
        self.ipv6 = a.ipv6;
        self.last_action = SteadyTime::now();
        d
    }
}
