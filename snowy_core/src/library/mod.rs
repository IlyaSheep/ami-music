use std::{collections::HashMap, fs::read_dir};

use serde::{Deserialize, Serialize};

use crate::{config::LibraryConfig, library::helper::is_rodio_supported, track::Track};

pub mod helper;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrackId(u64);

#[derive(Default)]
pub struct Library {
    pub tracks: HashMap<TrackId, Track>,
}

impl Library {
    pub fn load(&mut self, config: LibraryConfig) {
        self.tracks.clear();

        let mut id = 0;

        let track_vec: Vec<Track> = config
            .directories
            .iter()
            .filter(|path| path.is_dir())
            .flat_map(|dir| read_dir(dir).into_iter().flatten())
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter_map(|path| match is_rodio_supported(&path) {
                Ok(true) => Some(path),
                _ => None,
            })
            .filter_map(|path| Track::new(path.as_path()).ok())
            .collect();

        let mut tracks = HashMap::new();
        track_vec.into_iter().for_each(|t| {
            id += 1;
            tracks.insert(id, t);
        });
    }
}
