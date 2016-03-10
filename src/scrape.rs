use response::TrackerResponse;

#[derive(Debug)]
pub struct ScrapeResponse {
}

impl TrackerResponse for ScrapeResponse {
    fn to_bencode(&self) -> Vec<u8> {
        let resp = ben_map!{
            "success" => ben_bytes!("scrape!")
        };
        resp.encode()
    }
}
