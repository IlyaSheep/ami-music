use std::sync::Arc;

use ami_core::{
    library::TrackId, player::playback_snapshot::PlayerSnapshot, queue::Queue, track::Track,
};

#[derive(Debug, Default)]
pub struct AppStates {
    pub player_snapshot: PlayerSnapshot,
    pub queue_snapshot: Queue,
    pub library_snapshot: Vec<(TrackId, Arc<Track>)>,
    pub library_selected_index: Option<usize>,
}
