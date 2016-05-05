use tracker::announce::{Announce, AnnouncePeer};

use time::SteadyTime;
use std::net::{SocketAddrV4, SocketAddrV6};

pub struct Peer {
    pub id: String,
    pub uploaded: u64,
    pub downloaded: u64,
    pub left: u64,
    pub last_action: SteadyTime,
    pub ipv4: Option<SocketAddrV4>,
    pub ipv6: Option<SocketAddrV6>,
}

pub struct Delta {
    pub peer_id: String,
    pub upload: u64,
    pub download: u64,
    pub left: u64,
    pub passkey: Option<String>,
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
            passkey: a.passkey.clone(),
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
    pub fn new(peer_id: String, passkey: Option<String>) -> Delta {
        Delta {
            peer_id: peer_id,
            upload: 0,
            download: 0,
            left: 0,
            passkey: passkey,
        }
    }
}

#[test]
fn create_from_announce() {
    use tracker::announce::Action;

    let pid = String::from("pid");
    let ipv4 = None;
    let ipv6 = None;
    let ul = 1;
    let dl = 1;
    let left = 1;
    let announce =
        Announce {
            info_hash: String::from("hash"),
            peer_id: pid.clone(),
            passkey: None,
            ipv4: ipv4.clone(),
            ipv6: ipv6.clone(),
            ul: ul,
            dl: dl,
            left: left,
            action: Action::Seeding,
            numwant: 1,
            compact: true,
        };
    let peer = Peer::new(&announce);
    assert!(peer.uploaded == ul);
    assert!(peer.downloaded == dl);
    assert!(peer.left == left);
    assert!(peer.id == pid);
    assert!(peer.ipv4 == ipv4);
    assert!(peer.ipv6 == ipv6);
}

#[test]
fn peer_update() {
    use tracker::announce::Action;

    let pid = String::from("pid");
    let ipv4 = None;
    let ipv6 = None;
    let ul = 1;
    let dl = 1;
    let left = 1;

    let announce =
        Announce {
            info_hash: String::from("hash"),
            peer_id: pid.clone(),
            passkey: None,
            ipv4: ipv4.clone(),
            ipv6: ipv6.clone(),
            ul: ul,
            dl: dl,
            left: left,
            action: Action::Seeding,
            numwant: 1,
            compact: true,
        };

    let ipv4 = None;
    let ipv6 = None;
    let ul_2 = 2;
    let dl_2 = 2;
    let left_2 = 0;

    let announce2 =
        Announce {
            info_hash: String::from("hash"),
            peer_id: pid.clone(),
            passkey: None,
            ipv4: ipv4.clone(),
            ipv6: ipv6.clone(),
            ul: ul_2,
            dl: dl_2,
            left: left_2,
            action: Action::Seeding,
            numwant: 1,
            compact: true,
        };
    let mut peer = Peer::new(&announce);
    let delta = peer.update(&announce2);

    assert!(delta.upload == ul_2 - ul);
    assert!(delta.download == dl_2 - dl);
    assert!(delta.left == left - left_2);
}
