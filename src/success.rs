use response::TrackerResponse;
use announce::AnnounceResponse;
use scrape::ScrapeResponse;

#[derive(Debug)]
pub enum SuccessResponse {
    Announce(AnnounceResponse),
    Scrape(ScrapeResponse),
}

impl TrackerResponse for SuccessResponse {
    fn to_bencode(&self) -> Vec<u8> {
        match *self {
            SuccessResponse::Announce(ref a) => a.to_bencode(),
            SuccessResponse::Scrape(ref s) => s.to_bencode(),
        }
    }
}
