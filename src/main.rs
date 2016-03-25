extern crate hyper;
extern crate url;
extern crate concurrent_hashmap;
extern crate time;

#[macro_use]
extern crate bip_bencode;

mod route;
mod tracker;
mod torrent;
mod peer;
mod announce;
mod scrape;
mod error;
mod response;

use hyper::Server;
use route::RequestHandler;
use torrent::Torrent;
use std::sync::Arc;
use self::concurrent_hashmap::ConcHashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let server = Server::http("127.0.0.1:8000").unwrap();
    let torrents: ConcHashMap<String, Torrent> = Default::default();
    let tracker_arc = Arc::new(tracker::Tracker { torrents: torrents });
    let tracker_http = tracker_arc.clone();
    let tracker_reap = tracker_arc.clone();

    let handler = RequestHandler { tracker: tracker_http };
    let _guard = server.handle(handler).unwrap();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(1800));
            tracker_reap.reap();
        }
    });
    println!("Listening on port 8000");
}
