#[derive(Default)]
pub struct Metadata {
    pub length: u128,
    pub album: Option<String>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub disc_number: Option<u32>,
    pub genre: Option<String>,
}
