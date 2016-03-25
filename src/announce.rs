use response::TrackerResponse;
use std::net::{SocketAddrV4, SocketAddrV6};
use torrent::{Stats, Peers};

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
}

#[derive(Clone)]
pub enum Action {
    Seeding,
    Leeching,
    Completed,
    Stopped,
}

pub struct AnnounceResponse {
    pub stats: Stats,
    pub peers: Peers,
}

impl TrackerResponse for AnnounceResponse {
    fn to_bencode(&self) -> Vec<u8> {
        let resp = ben_map!{
            "peers" => ben_bytes!(&self.peers.peers4),
            "peers6" => ben_bytes!(&self.peers.peers6),
            "interval" => ben_int!(1800),
            "min interval" => ben_int!(900),
            "complete" => ben_int!(self.stats.complete),
            "downloaded" => ben_int!(self.stats.downloaded),
            "incomplete" => ben_int!(self.stats.incomplete)
        };
        resp.encode()
    }
}
