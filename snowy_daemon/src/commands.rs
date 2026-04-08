use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum Command {
    Queue(QueueCommand),
    Playback(PlaybackCommand),
    Library(LibraryCommand),
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum PlaybackCommand {
    Play,
    Pause,
    TogglePlay,
    SetPosition(Duration),
    Seek { offset_seconds: i64 },
    Restart,
    IncreaseVol { step: f32 },
    DecreaseVol { step: f32 },
    SetVolume { value: f32 },
    // SetLoop,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum QueueCommand {
    Enqueue { track_id: u64 },
    Prepend { track_id: u64 },
    Dequeue { position: usize, track_id: u64 },
    Next,
    Prev,
    Shuffle,
    Clear,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum LibraryCommand {
    Fetch,
}
