use std::{collections::HashMap, time::Duration};

use ami_core::{
    player::playback_snapshot::PlayerSnapshot, queue::queue_snapshot::QueueSnapshot, track::Track,
    track::track_id::TrackId,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ServerEvent {
    SendLibrary(HashMap<TrackId, Track>),
    SendQueue(QueueSnapshot),
    SendPlayerSnapshot(PlayerSnapshot),
    SendPlayerPosition(Duration),
}
