use response::TrackerResponse;
use response::error::ErrorResponse;
use response::success::SuccessResponse;
use tracker::Tracker;
use tracker::announce::{Action, Announce};
use tracker::scrape::Scrape;
use config::HttpConfig;

use hyper::server::{Request, Response, Handler};
use hyper::Server;
use hyper::uri::RequestUri::AbsolutePath;
use std::net::{IpAddr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::collections::HashMap;
use std::sync::Arc;
use url::{Url, UrlParser};
use std::str::FromStr;
use std::cmp;

pub struct RequestHandler {
    pub tracker: Arc<Tracker>,
    pub config: HttpConfig
}

impl Handler for RequestHandler {
    fn handle(&self, req: Request, res: Response) {
        let resp = match req.uri {
            AbsolutePath(ref path) => {
                println!("Req!");
                let base = Url::parse("http://localhost").unwrap();
                let url = UrlParser::new().base_url(&base).parse(path).unwrap();
                self.handle_url(&req, url)
            }
            _ => Err(ErrorResponse::BadAction),
        };
        res.send(serialize_resp(resp).as_slice()).unwrap();
    }
}

impl RequestHandler {
    pub fn start(tracker: Arc<Tracker>, config: HttpConfig) {
        let server = Server::http(config.listen_addr.as_str()).unwrap();
        let handler = RequestHandler { tracker: tracker, config: config };
        info!("HTTP interface listening on {}!", handler.config.listen_addr);
        let _guard = server.handle(handler).unwrap();
    }

    fn handle_url(&self, req: &Request, url: Url) -> Result<SuccessResponse, ErrorResponse> {
        if url.path().is_none() {
            return Err(ErrorResponse::BadAction);
        }
        let path = url.path().unwrap();
        let params = url.query_pairs();

        if cfg!(feature = "private") {
            if path.len() != 2 {
                Err(ErrorResponse::BadRequest)
            } else {
                if self.tracker.private.validate_passkey(&path[0]) {
                    Err(ErrorResponse::BadAuth)
                } else {
                    self.handle_req(req, &path[1], params, Some(path[0].clone()))
                }
            }
        } else {
            if path.len() != 1 {
                Err(ErrorResponse::BadRequest)
            } else {
                self.handle_req(req, &path[0], params, None)
            }
        }
    }

    fn handle_req(&self,
                  req: &Request,
                  path: &String,
                  params: Option<Vec<(String, String)>>,
                  passkey: Option<String>)
                  -> Result<SuccessResponse, ErrorResponse> {
        match &path[..] {
            "stats" => self.tracker.get_stats(),
            "announce" => {
                let announce = try!(self.request_to_announce(req, params, passkey));
                self.tracker.handle_announce(announce)
            }
            "scrape" => {
                let scrape = try!(self.request_to_scrape(params));
                self.tracker.handle_scrape(scrape)
            }
            _ => Err(ErrorResponse::BadAction),
        }
    }

    fn request_to_scrape(&self,
                         params: Option<Vec<(String, String)>>)
                         -> Result<Scrape, ErrorResponse> {
        if params.is_none() {
            return Err(ErrorResponse::BadRequest);
        }
        let param_vec = params.unwrap();

        let hashes = param_vec.into_iter()
                              .map(|(_, hash)| hash)
                              .collect();
        Ok(Scrape::new(hashes))
    }

    fn request_to_announce(&self,
                           req: &Request,
                           params: Option<Vec<(String, String)>>,
                           passkey: Option<String>)
                           -> Result<Announce, ErrorResponse> {
        if params.is_none() {
            return Err(ErrorResponse::BadRequest);
        }
        let param_vec = params.unwrap();
        if param_vec.len() > 10 {
            return Err(ErrorResponse::BadRequest);
        }

        let mut params = HashMap::new();
        for (key, val) in param_vec {
            params.insert(key, val);
        }

        let info_hash: String = try!(get_from_params(&params, String::from("info_hash")));
        if cfg!(feature = "private") {
            if !self.tracker.private.validate_torrent(&info_hash) {
                return Err(ErrorResponse::BadAuth);
            }
        }
        let pid: String = try!(get_from_params(&params, String::from("peer_id")));
        if cfg!(feature = "private") {
            if !self.tracker.private.validate_peer(&pid) {
                return Err(ErrorResponse::BadPeer);
            }
        }
        if info_hash.len() > 40 || pid.len() > 30 {
            return Err(ErrorResponse::BadRequest);
        }
        let ul = try!(get_from_params(&params, String::from("uploaded")));
        let dl = try!(get_from_params(&params, String::from("downloaded")));
        let left = try!(get_from_params(&params, String::from("left")));

        // IP parsing according to BEP 0007 with additional proxy forwarding check
        let port = try!(get_from_params(&params, String::from("port")));
        let (ipv4, ipv6) = get_ips(&params, req, &port);
        let action = match get_from_params::<String>(&params, String::from("event")) {
            Ok(ev_str) => {
                match &ev_str[..] {
                    "started" => get_action(left),
                    "stopped" => Action::Stopped,
                    "completed" => Action::Completed,
                    _ => get_action(left),
                }
            }
            Err(_) => get_action(left),
        };

        let numwant = cmp::min(get_from_params::<u8>(&params, String::from("numwant"))
                                   .unwrap_or(25),
                               25);

        let compact = get_from_params::<u8>(&params, String::from("compact")).unwrap_or(1) != 0;
        let announce = Announce {
            info_hash: info_hash,
            peer_id: pid,
            passkey: passkey,
            ipv4: ipv4,
            ipv6: ipv6,
            ul: ul,
            dl: dl,
            left: left,
            action: action,
            numwant: numwant,
            compact: compact,
        };

        if cfg!(feature = "private") {
            match self.tracker.private.validate_announce(&announce) {
                Some(e)  => return Err(ErrorResponse::BadPeer),
                None => ()
            }
        }

        Ok(announce)
    }
}

fn get_ips(params: &HashMap<String, String>,
           req: &Request,
           port: &u16)
           -> (Option<SocketAddrV4>, Option<SocketAddrV6>) {
    let port = *port;
    let default_ip = match req.headers.get_raw("X-Forwarded-For") {
        Some(bytes) => {
            match String::from_utf8(bytes[0].clone()) {
                Ok(ip_str) => {
                    match ip_str.parse::<IpAddr>() {
                        Ok(ip) => SocketAddr::new(ip, port),
                        Err(_) => req.remote_addr,
                    }
                }
                Err(_) => req.remote_addr,
            }
        }
        None => req.remote_addr,
    };
    let ip = match get_from_params(&params, String::from("ip")) {
        Ok(ip) => SocketAddr::new(ip, port),
        Err(_) => default_ip,
    };

    match ip {
        SocketAddr::V4(v4) => {
            let v6 = match get_socket(&params, String::from("ipv6"), port) {
                Some(sock) => {
                    match sock {
                        SocketAddr::V6(v6) => Some(v6),
                        _ => None,
                    }
                }
                None => None,
            };
            (Some(v4), v6)
        }
        SocketAddr::V6(v6) => {
            let v4 = match get_socket(&params, String::from("ipv4"), port) {
                Some(sock) => {
                    match sock {
                        SocketAddr::V4(v4) => Some(v4),
                        _ => None,
                    }
                }
                None => None,
            };
            (v4, Some(v6))
        }
    }
}

fn get_from_params<T: FromStr>(map: &HashMap<String, String>,
                               key: String)
                               -> Result<T, ErrorResponse> {
    match map.get(&key) {
        Some(res) => {
            match res.parse::<T>() {
                Ok(val) => Ok(val),
                Err(_) => Err(ErrorResponse::BadRequest),
            }
        }
        None => Err(ErrorResponse::BadRequest),
    }
}

fn get_socket(params: &HashMap<String, String>, key: String, port: u16) -> Option<SocketAddr> {
    let ip = get_from_params(params, key.clone());
    let socket = get_from_params(params, key.clone());
    match (ip, socket) {
        (Err(_), Err(_)) => None,
        (Ok(ip), Err(_)) => Some(SocketAddr::new(ip, port)),
        (Err(_), Ok(sock)) => Some(sock),
        _ => None,
    }
}

fn get_action(left: u64) -> Action {
    if left == 0 {
        Action::Seeding
    } else {
        Action::Leeching
    }
}

fn serialize_resp(result: Result<SuccessResponse, ErrorResponse>) -> Vec<u8> {
    match result {
        Ok(resp) => resp.http_resp(),
        Err(err) => err.to_bencode(),
    }
}
