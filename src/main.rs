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

fn main() {
    let server = Server::http("127.0.0.1:8000").unwrap();
    let torrents: ConcHashMap<String, Torrent> = Default::default();
    let tracker = Arc::new(tracker::Tracker {
        torrents: torrents
    }).clone();

    let handler = RequestHandler {
        tracker: tracker
    };
    let _guard = server.handle(handler).unwrap();
    println!("Listening on port 8000");
}
