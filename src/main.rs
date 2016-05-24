extern crate hyper;
extern crate url;
extern crate time;
extern crate env_logger;
extern crate getopts;
extern crate toml;
extern crate parking_lot;

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
use config::MainConfig;
use getopts::Options;
use std::env;
use std::io::Read;
use std::fs::File;

fn main() {
    env_logger::init().unwrap();
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print the help menu");
    opts.optopt("c", "config", "path to config file", "/path/to/config.toml");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let config = matches.opt_str("c")
        .map_or(None, |path| File::open(path).ok())
        .map_or(None, |mut f| {
            let mut s = String::new();
            if f.read_to_string(&mut s).is_err() {
                None
            } else {
                Some(s)
            }
        })
        .map_or(None, |s| toml::Parser::new(&s).parse())
        .map_or(None, |toml| Some(MainConfig::from_toml(toml)))
        .unwrap_or_else(|| {
            println!("No config file provided, or failed to parse config file! Falling back to default config.");
            Default::default()
        });

    run_tracker(config);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn run_tracker(config: MainConfig) {
    let tracker = Tracker::new(config.tracker.clone(), config.private.clone());
    let tracker_arc = Arc::new(tracker);
    Tracker::start_updaters(tracker_arc.clone());
    http::RequestHandler::start(tracker_arc.clone(), config.http.clone());
}
