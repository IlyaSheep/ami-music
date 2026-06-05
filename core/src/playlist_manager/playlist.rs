use crate::track::track_id::TrackId;

#[derive(Default, Debug, Clone)]
pub struct Playlist {
    pub name: String,
    pub tracks: Vec<TrackId>,
}

impl Playlist {
    pub fn new(name: String) -> Playlist {
        Playlist {
            name,
            tracks: Vec::new(),
        }
    }
}
