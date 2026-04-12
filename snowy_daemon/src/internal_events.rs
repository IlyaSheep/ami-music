use crate::states::AppState;
use anyhow::Result;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum InternalEvent {
    PlayerEmpty,
}

pub async fn handle_internal_event(
    event: InternalEvent,
    state: &mut AppState,
    connection_tx: &broadcast::Sender<String>,
) -> Result<()> {
    match event {
        InternalEvent::PlayerEmpty => {
            print!("player empty");
            let mut orchestrator = state.orchestrator.lock().await;
            orchestrator.queue.next();
            if let Some(current) = orchestrator.queue.current_track.clone() {
                orchestrator.playback.load_track(&current.pathbuf)?;
            }
        }
    }

    Ok(())
}
