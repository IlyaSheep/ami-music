use crate::{
    commands::{Command, LibraryCommand, PlaybackCommand, QueueCommand},
    states::AppState,
};

pub fn handle_command(command: Command, state: &mut AppState) {
    match command {
        Command::Playback(cmd) => handle_playback_command(cmd),
        Command::Queue(cmd) => handle_queue_command(cmd),
        Command::Library(cmd) => handle_library_command(cmd),
    }
}

pub fn handle_playback_command(command: PlaybackCommand) {
    match command {
        PlaybackCommand::Play => todo!(),
        PlaybackCommand::Pause => todo!(),
        PlaybackCommand::TogglePlay => todo!(),
        PlaybackCommand::SetPosition(_) => todo!(),
        PlaybackCommand::Seek { .. } => todo!(),
        PlaybackCommand::Restart => todo!(),
        PlaybackCommand::IncreaseVol { .. } => todo!(),
        PlaybackCommand::DecreaseVol { .. } => todo!(),
        PlaybackCommand::SetVolume { .. } => todo!(),
    }
}

pub fn handle_queue_command(command: QueueCommand) {
    match command {
        QueueCommand::Enqueue { .. } => todo!(),
        QueueCommand::Prepend { .. } => todo!(),
        QueueCommand::Dequeue { .. } => todo!(),
        QueueCommand::Next => todo!(),
        QueueCommand::Prev => todo!(),
        QueueCommand::Shuffle => todo!(),
        QueueCommand::Clear => todo!(),
    }
}

pub fn handle_library_command(command: LibraryCommand) {
    match command {
        LibraryCommand::Fetch => todo!(),
    }
}
