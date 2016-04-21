extern crate hyper;
extern crate url;
extern crate time;
extern crate env_logger;
extern crate toml;

#[macro_use]
extern crate bip_bencode;

#[macro_use]
extern crate log;

mod tracker;
mod http;
mod response;
mod private;

use tracker::Tracker;
use std::sync::Arc;

fn main() {
    if cfg!(debug_assertions) {
        env_logger::init().unwrap();
    }
    let tracker = Tracker::new();
    let tracker_arc = Arc::new(tracker);
    Tracker::start_updaters(tracker_arc.clone());
    http::RequestHandler::start(tracker_arc.clone());
}
