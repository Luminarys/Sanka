use tracker::Tracker;
use response::TrackerResponse;
use error::ErrorResponse;

use hyper::server::{Request, Response, Handler};
use hyper::uri::RequestUri::AbsolutePath;
use std::sync::Arc;
use url::form_urlencoded;

pub struct RequestHandler {
    pub tracker: Arc<Tracker>,
}

impl Handler for RequestHandler {
    fn handle(&self, req: Request, res: Response) {
        let resp = match req.uri {
            AbsolutePath(ref path) => {
                match path.find('?') {
                    Some(i) => {
                        let (action, param_str) = path.split_at(i + 1);
                        let param_vec = form_urlencoded::parse(param_str.as_bytes());
                        match action {
                            "/announce?" => {
                                bencode_result(self.tracker.handle_announce(&req, param_vec))
                            }
                            "/scrape?" => bencode_result(self.tracker.handle_scrape(param_vec)),
                            _ => bencode_resp(ErrorResponse::BadAction),
                        }
                    }
                    None => bencode_resp(ErrorResponse::BadRequest),
                }
            }
            _ => bencode_resp(ErrorResponse::BadAction),
        };
        res.send(resp.as_slice()).unwrap();
    }
}

fn bencode_result<S: TrackerResponse, E: TrackerResponse>(result: Result<S, E>) -> Vec<u8> {
    match result {
        Ok(resp) => resp.to_bencode(),
        Err(err) => err.to_bencode(),
    }
}

fn bencode_resp<T: TrackerResponse>(resp: T) -> Vec<u8> {
    resp.to_bencode()
}
