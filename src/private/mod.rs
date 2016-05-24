use config::PrivateConfig;
use tracker::peer::Delta;
use tracker::announce::Announce;
use response::error::ErrorResponse;

use parking_lot::{Mutex, RwLock};
use std::collections::HashSet;
use std::mem;

#[allow(dead_code, unused_variables)]
pub struct PrivateTracker {
    deltas: Mutex<Vec<Delta>>,
    torrents: RwLock<HashSet<String>>,
    peers: RwLock<Vec<String>>,
    passkeys: RwLock<HashSet<String>>,
    pub config: PrivateConfig
}

impl Default for PrivateTracker {
    fn default() -> PrivateTracker {
        PrivateTracker::new(Default::default())
    }
}

#[allow(unused_variables)]
impl PrivateTracker {
    pub fn new(config: PrivateConfig) -> PrivateTracker {
        let deltas = Mutex::new(Default::default());
        let torrents = RwLock::new(Default::default());
        let peers = RwLock::new(Default::default());
        let passkeys = RwLock::new(Default::default());

        // Fill in implementation here

        PrivateTracker {
            deltas: deltas,
            torrents: torrents,
            peers: peers,
            passkeys: passkeys,
            config: config
        }
    }

    pub fn add_announce(&self, delta: Delta) {
        let mut deltas = self.deltas.lock();
        deltas.push(delta);
    }

    pub fn flush(&self) {
        let mut deltas = Vec::new();
        {
            let mut old_deltas = self.deltas.lock();
            mem::swap(&mut deltas, &mut *old_deltas);
        }
        // Fill in implementation here
    }

    pub fn validate_passkey(&self, passkey: &String) -> bool {
        // Fill in implementation here
        true
    }

    pub fn validate_peer(&self, id: &String) -> bool {
        // Fill in implementation here
        true
    }

    pub fn validate_torrent(&self, hash: &String) -> bool {
        // Fill in implementation here
        true
    }

    pub fn validate_announce(&self, announce: &Announce) -> Option<ErrorResponse> {
        // Fill in implementation here
        None
    }

    pub fn update(&self) {
        // Fill in implementation here
    }
}
