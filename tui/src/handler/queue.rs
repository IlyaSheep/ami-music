use ami_core::library::TrackId;
use ami_daemon::commands::{Command, QueueCommand};
use tokio::sync::mpsc::UnboundedSender;

pub fn enqueue(track_id: TrackId, command_tx: UnboundedSender<Command>) {
    let _ = command_tx.send(Command::Queue(QueueCommand::Enqueue { track_id }));
}

pub fn next(command_tx: UnboundedSender<Command>) {
    let _ = command_tx.send(Command::Queue(QueueCommand::Next));
}

pub fn prev(command_tx: UnboundedSender<Command>) {
    let _ = command_tx.send(Command::Queue(QueueCommand::Prev));
}
