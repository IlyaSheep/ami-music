use anyhow::Result;
use tokio::sync::broadcast;

use crate::{
    app::SharedState,
    commands::{Command, LibraryCommand, PlaybackCommand, QueueCommand},
    events::ServerEvent,
};

pub async fn handle_command(
    command: Command,
    state: SharedState,
    connection_tx: &broadcast::Sender<String>,
) -> Result<()> {
    match command {
        Command::Playback(cmd) => handle_playback_command(cmd, state, connection_tx).await,
        Command::Queue(cmd) => handle_queue_command(cmd, state, connection_tx).await,
        Command::Library(cmd) => handle_library_command(cmd, state, connection_tx).await,
    }
}

pub async fn handle_playback_command(
    command: PlaybackCommand,
    shared_state: SharedState,
    tx: &broadcast::Sender<String>,
) -> Result<()> {
    let no_broadcast = matches!(command, PlaybackCommand::Seek { .. });

    match command {
        PlaybackCommand::Play => shared_state.read().await.play()?,

        PlaybackCommand::Pause => shared_state.read().await.pause(),

        PlaybackCommand::TogglePlay => shared_state.read().await.toggle_play()?,

        PlaybackCommand::SetPosition(pos) => shared_state.read().await.set_position(pos)?,

        PlaybackCommand::Seek { offset_seconds } => {
            shared_state.read().await.seek(offset_seconds)?
        }

        PlaybackCommand::Restart => shared_state.read().await.rewind()?,

        PlaybackCommand::IncreaseVol { step } => shared_state.read().await.increase_volume(step),

        PlaybackCommand::DecreaseVol { step } => shared_state.read().await.decrease_volume(step),

        PlaybackCommand::SetVolume { value } => shared_state.read().await.set_volume(value),

        PlaybackCommand::GetSnapshot => {}
    };

    if !no_broadcast {
        let event =
            ServerEvent::SendPlayerSnapshot(shared_state.read().await.get_player_snapshot());
        let json = serde_json::to_string(&event)?;
        let _ = tx.send(json);
    }

    Ok(())
}

pub async fn handle_queue_command(
    command: QueueCommand,
    shared_state: SharedState,
    tx: &broadcast::Sender<String>,
) -> Result<()> {
    match command {
        QueueCommand::Enqueue { track_id } => {
            let mut orchestrator = shared_state.write().await;
            orchestrator.enqueue(track_id).await?;
            let event = ServerEvent::SendPlayerSnapshot(orchestrator.get_player_snapshot());
            let json = serde_json::to_string(&event)?;
            let _ = tx.send(json);
        }
        QueueCommand::Prepend { track_id } => shared_state.write().await.prepend(track_id),
        QueueCommand::Dequeue { index } => shared_state.write().await.dequeue(index),
        QueueCommand::PlayNow { track_id } => shared_state.write().await.play_now(track_id).await?,
        QueueCommand::Next => {
            shared_state.write().await.next().await?;
        }
        QueueCommand::Prev => shared_state.write().await.prev().await?,
        QueueCommand::Shuffle => shared_state.write().await.shuffle(),
        QueueCommand::Clear => shared_state.write().await.clear(),
        QueueCommand::SetLoopMode(loop_mode) => shared_state.write().await.set_loop_mode(loop_mode),
        QueueCommand::CycleLoopMode => shared_state.write().await.cycle_loop_mode(),
        QueueCommand::Fetch => {}
    };

    let event = ServerEvent::SendQueue(shared_state.read().await.clone_queue());
    let json = serde_json::to_string(&event)?;
    let _ = tx.send(json);

    Ok(())
}

pub async fn handle_library_command(
    command: LibraryCommand,
    state: SharedState,
    tx: &broadcast::Sender<String>,
) -> Result<()> {
    match command {
        LibraryCommand::Fetch => {
            let event = ServerEvent::SendLibrary(state.read().await.clone_library());
            let json = serde_json::to_string(&event)?;
            let _ = tx.send(json);
        }
    }

    Ok(())
}
