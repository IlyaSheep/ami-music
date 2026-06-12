use std::time::Duration;

use ami_core::{queue::loop_mode::LoopMode, track::track_id::TrackId};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, TS)]
#[ts(export, export_to = "commands.ts")]
pub enum Command {
    Queue(QueueCommand),
    Playback(PlaybackCommand),
    Library(LibraryCommand),
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, TS)]
#[ts(export, export_to = "commands.ts")]
pub enum PlaybackCommand {
    Play,
    Pause,
    TogglePlay,
    SetPosition {
        #[ts(type = "{ secs: number, nanos: number }")]
        position: Duration,
    },
    Seek {
        offset_seconds: i64,
    },
    Restart,
    IncreaseVol {
        step: f32,
    },
    DecreaseVol {
        step: f32,
    },
    SetVolume {
        value: f32,
    },
    GetSnapshot,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, TS)]
#[ts(export, export_to = "commands.ts")]
pub enum QueueCommand {
    Enqueue { track_id: TrackId },
    Prepend { track_id: TrackId },
    Dequeue { index: usize },
    PlayNow { track_id: TrackId },
    Next,
    Prev,
    Shuffle,
    Clear,
    SetLoopMode(LoopMode),
    CycleLoopMode,
    Fetch,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, TS)]
#[ts(export, export_to = "commands.ts")]
pub enum LibraryCommand {
    Fetch,
}
