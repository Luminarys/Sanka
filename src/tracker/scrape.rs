use tracker::torrent::Stats;
use std::collections::HashMap;

pub struct Scrape {
    pub torrents: Vec<String>
}

#[derive(Debug)]
pub struct ScrapeResponse {
    pub torrents: HashMap<String, Stats>,
}


impl Scrape {
    pub fn new(torrents: Vec<String>) -> Scrape {
        Scrape { torrents: torrents }
    }
}
