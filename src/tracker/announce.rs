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

#[derive(Debug)]
pub struct AnnounceResponse<'a> {
    pub stats: Stats,
    pub peers: Peers<'a>,
    pub compact: bool,
}
