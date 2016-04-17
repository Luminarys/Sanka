use std::net::{SocketAddrV4, SocketAddrV6};
use std::sync::MutexGuard;
use std::collections::HashMap;

use tracker::torrent::{Stats, Peers, Torrent};

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

pub struct AnnounceResponse<'a> {
    announce: Announce,
    guard: MutexGuard<'a, HashMap<String, Torrent>>,
}

impl<'a> AnnounceResponse<'a> {
    pub fn new(announce: Announce,
               guard: MutexGuard<HashMap<String, Torrent>>)
               -> AnnounceResponse {
        AnnounceResponse {
            announce: announce,
            guard: guard
        }
    }

    pub fn peers(&self) -> Peers {
        let t = self.guard.get(&self.announce.info_hash).unwrap();
        t.get_peers(self.announce.numwant.clone(), self.announce.action.clone())
    }

    pub fn stats(&self) -> Stats {
        let t = self.guard.get(&self.announce.info_hash).unwrap();
        t.get_stats()
    }

    pub fn compact(&self) -> bool {
        self.announce.compact
    }
}
