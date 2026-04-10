use crate::states::AppState;
use anyhow::Result;
use tokio::sync::broadcast;

pub enum InternalEvent {
    PlayerEmpty,
}

pub async fn handle_internal_event(
    event: InternalEvent,
    state: &mut AppState,
    tx: &broadcast::Sender<String>,
) -> Result<()> {
    match event {
        InternalEvent::PlayerEmpty => {
            let mut orchestrator = state.orchestrator.lock().await;
            while orchestrator.queue.current_track.is_some()
                || !orchestrator.queue.next_tracks.is_empty()
            {
                if let Some(track) = orchestrator.queue.current_track.clone() {
                    orchestrator.playback.load_track(track.pathbuf.as_path())?;
                    break;
                } else {
                    orchestrator.queue.next();
                }
            }

        }
    }

    Ok(())
}
