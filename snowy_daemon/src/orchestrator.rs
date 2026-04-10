use anyhow::Result;
use snowy_core::{player::Playback, queue::Queue};
use tokio::sync::broadcast;

use crate::internal_events::InternalEvent;

pub struct Orchestrator {
    pub playback: Playback,
    pub queue: Queue,
    pub internal_event_tx: broadcast::Sender<InternalEvent>,
}

impl Orchestrator {
    pub fn new(tx: broadcast::Sender<InternalEvent>) -> Result<Self> {
        Ok(Orchestrator {
            playback: Playback::new()?,
            queue: Queue::default(),
            internal_event_tx: tx,
        })
    }
}
