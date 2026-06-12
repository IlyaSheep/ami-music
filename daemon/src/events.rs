use std::{collections::HashMap, time::Duration};

use ami_core::{
    player::playback_snapshot::PlayerSnapshot, queue::queue_snapshot::QueueSnapshot, track::Track,
    track::track_id::TrackId,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(tag = "type", content = "data")]
#[ts(export, export_to = "server_event.ts")]
pub enum ServerEvent {
    SendLibrary(HashMap<TrackId, Track>),
    SendQueue(QueueSnapshot),
    SendPlayerSnapshot(PlayerSnapshot),
    SendPlayerPosition {
        #[ts(type = "{ secs: number, nanos: number }")]
        position: Duration,
    },
}
