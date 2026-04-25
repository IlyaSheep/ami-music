use std::sync::Arc;

use ami_daemon::commands::{Command, QueueCommand};
use tokio::sync::{Mutex, mpsc::UnboundedSender};

use crate::state::DaemonStates;

pub async fn enqueue(command_tx: UnboundedSender<Command>, states: Arc<Mutex<DaemonStates>>) {
    let states = states.lock().await;
    if let Some(track) = states.library_snapshot.get(states.library_selected_index) {
        let _ = command_tx.send(Command::Queue(QueueCommand::Enqueue { track_id: track.0 }));
    }
}

pub fn next(command_tx: UnboundedSender<Command>) {
    let _ = command_tx.send(Command::Queue(QueueCommand::Next));
}

pub fn prev(command_tx: UnboundedSender<Command>) {
    let _ = command_tx.send(Command::Queue(QueueCommand::Prev));
}
