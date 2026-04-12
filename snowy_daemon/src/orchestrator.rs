use std::{sync::Arc, time::Duration};

use anyhow::Result;
use rodio::Player;
use snowy_core::{player::Playback, queue::Queue};
use tokio::{sync::broadcast, time::sleep};

use crate::internal_events::InternalEvent;

pub struct Orchestrator {
    pub playback: Arc<Playback>,
    pub queue: Arc<Queue>,
    pub internal_event_tx: Arc<broadcast::Sender<InternalEvent>>,
}

impl Orchestrator {
    pub fn new(tx: Arc<broadcast::Sender<InternalEvent>>) -> Result<Self> {
        Ok(Orchestrator {
            playback: Arc::new(Playback::new()?),
            queue: Arc::new(Queue::default()),
            internal_event_tx: tx,
        })
    }

    pub async fn watch_track_end(
        player: Arc<Player>,
        queue: Arc<Queue>,
        internal_event_tx: Arc<broadcast::Sender<InternalEvent>>,
    ) {
        loop {
            print!("player empty loop");
            if player.empty() {
                let _ = internal_event_tx.send(InternalEvent::PlayerEmpty);
                while queue.is_empty() {
                    sleep(Duration::from_millis(100)).await;
                }
            }
            sleep(Duration::from_millis(50)).await;
        }
    }
}
