use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    #[default]
    Stopped,
}
