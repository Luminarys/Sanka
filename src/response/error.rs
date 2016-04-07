use response::TrackerResponse;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ErrorResponse {
    BadAuth,
    BadRequest,
    BadAction,
}

impl TrackerResponse for ErrorResponse {
    fn to_bencode(&self) -> Vec<u8> {
        let resp = match *self {
            ErrorResponse::BadAuth => {
                ben_map!{
                    "failure reason" => ben_bytes!("Improper authentication.")
                }
            }
            ErrorResponse::BadRequest => {
                ben_map!{
                    "failure reason" => ben_bytes!("Improper request.")
                }
            }
            ErrorResponse::BadAction => {
                ben_map!{
                    "failure reason" => ben_bytes!("Improper action.")
                }
            }
        };
        resp.encode()
    }
}

impl From<ParseIntError> for ErrorResponse {
    fn from(_err: ParseIntError) -> ErrorResponse {
        ErrorResponse::BadRequest
    }
}
