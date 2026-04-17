use ami_daemon::commands::{Command, PlaybackCommand};
use tokio::sync::mpsc::UnboundedSender;

const SEEK_STEP_SEC: i64 = 5;

pub fn toggle_play(command_tx: UnboundedSender<Command>) {
    let _ = command_tx.send(Command::Playback(PlaybackCommand::TogglePlay));
}

pub fn seek_forward(command_tx: UnboundedSender<Command>) {
    let _ = command_tx.send(Command::Playback(PlaybackCommand::Seek {
        offset_seconds: SEEK_STEP_SEC,
    }));
}

pub fn seek_backward(command_tx: UnboundedSender<Command>) {
    let _ = command_tx.send(Command::Playback(PlaybackCommand::Seek {
        offset_seconds: -SEEK_STEP_SEC,
    }));
}
