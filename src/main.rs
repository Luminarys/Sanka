extern crate hyper;
extern crate url;
extern crate concurrent_hashmap;
extern crate time;

#[macro_use]
extern crate bip_bencode;

#[macro_use]
extern crate log;
extern crate env_logger;

mod tracker;
mod route;
mod announce;
mod scrape;
mod error;
mod success;
mod response;
mod stats;

use hyper::Server;
use route::RequestHandler;
use tracker::Tracker;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    if cfg!(debug_assertions) {
        env_logger::init().unwrap();
    }
    let server = Server::http("127.0.0.1:8000").unwrap();
    let tracker = Tracker::new();
    let tracker_arc = Arc::new(tracker);
    let tracker_http = tracker_arc.clone();
    let tracker_reap = tracker_arc.clone();

    let handler = RequestHandler { tracker: tracker_http };
    let _guard = server.handle(handler).unwrap();
    thread::spawn(move || {
        info!("Starting reaper!");
        loop {
            thread::sleep(Duration::from_secs(1800));
            tracker_reap.reap();
        }
    });
    info!("Listening on port 8000");
}
