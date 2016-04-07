use response::TrackerResponse;
use tracker::torrent::Stats;
use std::collections::{BTreeMap, HashMap};
use bip_bencode::Bencode;
use std::convert::AsRef;

pub struct Scrape {
    pub torrents: Vec<String>
}

impl Scrape {
    pub fn new(torrents: Vec<String>) -> Scrape {
        Scrape { torrents: torrents }
    }
}

#[derive(Debug)]
pub struct ScrapeResponse {
    pub torrents: HashMap<String, Stats>,
}

impl TrackerResponse for ScrapeResponse {
    fn to_bencode(&self) -> Vec<u8> {
        let mut resp = BTreeMap::new();
        let mut torrents = BTreeMap::new();
        for (key, val) in self.torrents.iter() {
            let torrent = ben_map!{
                "complete" => ben_int!(val.complete),
                "downloaded" => ben_int!(val.downloaded),
                "incomplete" => ben_int!(val.incomplete)
            };
            torrents.insert(AsRef::as_ref(key), torrent);
        }
        resp.insert("files", Bencode::Dict(torrents));
        Bencode::Dict(resp).encode()
    }
}
