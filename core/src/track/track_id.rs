use std::{
    hash::{DefaultHasher, Hash, Hasher},
    ops::Deref,
    path::Path,
};

use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use ts_rs::TS;

#[serde_as]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export, export_to = "track_id.ts", type = "string")]
pub struct TrackId(#[serde_as(as = "DisplayFromStr")] pub u64);

impl TrackId {
    pub fn from_path(path: &Path) -> TrackId {
        let mut h = DefaultHasher::new();
        path.hash(&mut h);
        TrackId(h.finish())
    }
}

impl Deref for TrackId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
