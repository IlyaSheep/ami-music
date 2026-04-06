use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
}
