pub trait TrackerResponse {
    fn to_bencode(&self) -> Vec<u8>;
}
