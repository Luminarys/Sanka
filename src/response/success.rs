use tracker::announce::AnnounceResponse;
use tracker::scrape::ScrapeResponse;
use tracker::stats::StatsResponse;

use bip_bencode::Bencode;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum SuccessResponse {
    Announce(AnnounceResponse),
    Scrape(ScrapeResponse),
    Stats(StatsResponse),
}

impl SuccessResponse {
    pub fn http_resp(&self) -> Vec<u8> {
        match *self {
            SuccessResponse::Announce(ref a) => bencode_announce(a),
            SuccessResponse::Scrape(ref s) => bencode_scrape(s),
            SuccessResponse::Stats(ref s) => display_stats(s),
        }
    }
}

fn bencode_announce(a: &AnnounceResponse) -> Vec<u8> {
    let benc = ben_map!{
       "peers" => ben_bytes!(&a.peers.peers4),
       "peers6" => ben_bytes!(&a.peers.peers6),
       "interval" => ben_int!(1800),
       "min interval" => ben_int!(900),
       "complete" => ben_int!(a.stats.complete),
       "downloaded" => ben_int!(a.stats.downloaded),
       "incomplete" => ben_int!(a.stats.incomplete)
    };
    benc.encode()
}

fn bencode_scrape(s: &ScrapeResponse) -> Vec<u8> {
    let mut resp = BTreeMap::new();
    let mut torrents = BTreeMap::new();
    for (key, val) in s.torrents.iter() {
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
fn display_stats(s: &StatsResponse) -> Vec<u8> {
    String::from(format!("Announces/s: {}\nScrapes/s: {}\nTorrents: {}\nPeers: {}",
                         s.announce_rate,
                         s.scrape_rate,
                         s.torrents,
                         s.peers))
        .into_bytes()
}
