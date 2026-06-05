use std::{collections::HashMap, sync::Arc};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use walkdir::WalkDir;

use crate::{
    config::LibraryConfig,
    library::helper::is_rodio_supported,
    track::{Track, track_id::TrackId},
};

pub mod helper;

#[cfg(test)]
pub mod tests;

#[derive(Default, Debug, Clone)]
pub struct Library {
    pub track_map: HashMap<TrackId, Arc<Track>>,
}

impl Library {
    pub fn load(&mut self, config: LibraryConfig) {
        self.track_map.clear();

        let track_vec: Vec<Track> = config
            .directories
            .par_iter()
            .filter(|path| path.is_dir())
            .flat_map(|dir| {
                WalkDir::new(dir)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.file_type().is_file())
                    .map(|e| e.into_path())
                    .collect::<Vec<_>>()
            })
            .filter_map(|path| match is_rodio_supported(&path) {
                Ok(true) => Some(path),
                _ => None,
            })
            .filter_map(|path| Track::new(path.as_path()).ok())
            .collect();

        let mut track_map = HashMap::new();
        track_vec.into_iter().for_each(|t| {
            track_map.insert(t.id, Arc::new(t));
        });

        self.track_map = track_map;
    }
}
