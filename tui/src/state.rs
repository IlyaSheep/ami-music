use ami_core::{library::Library, player::playback_snapshot::PlayerSnapshot, queue::Queue};

#[derive(Debug, Default)]
pub struct AppStates {
    pub player_snapshot: PlayerSnapshot,
    pub queue_snapshot: Queue,
    pub library_snapshot: Library,
}
