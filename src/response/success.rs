use tracker::announce::AnnounceResponse;
use tracker::scrape::ScrapeResponse;
use tracker::stats::StatsResponse;

use bip_bencode::Bencode;
use std::collections::BTreeMap;

pub enum SuccessResponse<'a> {
    Announce(AnnounceResponse<'a>),
    Scrape(ScrapeResponse),
    Stats(StatsResponse),
}

impl<'a> SuccessResponse<'a> {
    pub fn http_resp(&'a self) -> Vec<u8> {
        match *self {
            SuccessResponse::Announce(ref a) => bencode_announce(a),
            SuccessResponse::Scrape(ref s) => bencode_scrape(s),
            SuccessResponse::Stats(ref s) => display_stats(s),
        }
    }
}

fn bencode_announce(a: &AnnounceResponse) -> Vec<u8> {
    let peers = a.peers();
    let stats = a.stats();

    if a.compact() {
        let mut peer_bytes = Vec::with_capacity(6 * peers.peers4.len() as usize);
        for p in peers.peers4.iter() {
            peer_bytes.extend(p.get_ipv4_bytes().unwrap());
        }

        let mut peer6_bytes = Vec::with_capacity(18 * peers.peers6.len() as usize);
        for p in peers.peers6.iter() {
            peer6_bytes.extend(p.get_ipv6_bytes().unwrap());
        }

        let benc = ben_map!{
           "peers" => ben_bytes!(&peer_bytes),
           "peers6" => ben_bytes!(&peer6_bytes),
           "interval" => ben_int!(1800),
           "min interval" => ben_int!(900),
           "complete" => ben_int!(stats.complete),
           "downloaded" => ben_int!(stats.downloaded),
           "incomplete" => ben_int!(stats.incomplete)
        };
        benc.encode()
    } else {
        let temp_peers: Vec<_> = peers.peers4.iter().map(|p| {
            (p.id.clone(), p.get_ipv4_str().unwrap(), p.ipv4.unwrap().port().to_string())
        }).collect();
        let peers_benc: Vec<_> = temp_peers.iter().map(|&(ref id, ref ip, ref port)| {
            ben_map!{
                "peer id" => ben_bytes!(id),
                "ip" => ben_bytes!(ip),
                "port" => ben_bytes!(port)
            }
        }).collect();

        let temp_peers6: Vec<_> = peers.peers6.iter().map(|p| {
            (p.id.clone(), p.get_ipv6_str().unwrap(), p.ipv6.unwrap().port().to_string())
        }).collect();
        let peers6_benc: Vec<_> = temp_peers6.iter().map(|&(ref id, ref ip, ref port)| {
            ben_map!{
                "peer id" => ben_bytes!(id),
                "ip" => ben_bytes!(ip),
                "port" => ben_bytes!(port)
            }
        }).collect();

        let benc = ben_map!{
           "peers" => Bencode::List(peers_benc),
           "peers6" => Bencode::List(peers6_benc),
           "interval" => ben_int!(1800),
           "min interval" => ben_int!(900),
           "complete" => ben_int!(stats.complete),
           "downloaded" => ben_int!(stats.downloaded),
           "incomplete" => ben_int!(stats.incomplete)
        };
        benc.encode()
    }
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
