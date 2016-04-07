pub mod error;
pub mod success;

pub trait TrackerResponse {
    fn to_bencode(&self) -> Vec<u8>;
}
