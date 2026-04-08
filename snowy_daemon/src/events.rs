use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use snowy_core::{library::TrackId, track::Track};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ServerEvent {
    SendLibrary(HashMap<TrackId, Track>),
}
