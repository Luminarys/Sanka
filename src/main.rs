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
mod config;

use tracker::Tracker;
use std::sync::Arc;

fn main() {
    if cfg!(debug_assertions) {
        env_logger::init().unwrap();
    }
    let toml = r#"
    [test]
    foo = "bar"
"#;

    let value = toml::Parser::new(toml).parse().unwrap();
    println!("{:?}", value);
    let tracker = Tracker::default();
    let tracker_arc = Arc::new(tracker);
    Tracker::start_updaters(tracker_arc.clone());
    http::RequestHandler::start(tracker_arc.clone(), Default::default());
}
