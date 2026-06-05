use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    ops::Deref,
    path::Path,
    sync::Arc,
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use walkdir::WalkDir;

use crate::{config::LibraryConfig, library::helper::is_rodio_supported, track::Track};

pub mod helper;

#[cfg(test)]
pub mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export, export_to = "track_id.ts")]
pub struct TrackId(pub u64);

impl TrackId {
    pub fn from_path(path: &Path) -> TrackId {
        let mut h = DefaultHasher::new();
        path.hash(&mut h);
        TrackId(h.finish())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTrackIdError;

impl Deref for TrackId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
