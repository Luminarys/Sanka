use response::TrackerResponse;
use std::num::ParseIntError;
//use std::error::Error;
//use std::fmt::Display;
//use std::fmt;

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
            },
            ErrorResponse::BadRequest => {
                ben_map!{
                    "failure reason" => ben_bytes!("Improper request.")
                }
            },
            ErrorResponse::BadAction => {
                ben_map!{
                    "failure reason" => ben_bytes!("Improper action.")
                }
            }
        };
        resp.encode()
    }
}

//impl Display for ErrorResponse {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        match *self {
//            ErrorResponse::BadAction => write!(f, "Improper action."),
//            ErrorResponse::BadAuth => write!(f, "Improper authentication."),
//            ErrorResponse::BadRequest => write!(f, "Improper request."),
//        }
//    }
//}
//
//impl Error for ErrorResponse {
//    fn description(&self) -> &str {
//        match *self {
//            ErrorResponse::BadAction => "Improper action.",
//            ErrorResponse::BadAuth => "Improper authentication.",
//            ErrorResponse::BadRequest => "Improper request.",
//        }
//    }
//}

impl From<ParseIntError> for ErrorResponse {
    fn from(_err: ParseIntError) -> ErrorResponse {
        ErrorResponse::BadRequest
    }
}
