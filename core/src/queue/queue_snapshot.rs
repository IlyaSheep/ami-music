use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{queue::loop_mode::LoopMode, track::Track};

#[derive(Default, Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "queue_snapshot.ts")]
pub struct QueueSnapshot {
    pub loop_mode: LoopMode,
    pub current_track: Option<Track>,
    pub previous_tracks: Vec<Track>,
    pub next_tracks: Vec<Track>,
}
