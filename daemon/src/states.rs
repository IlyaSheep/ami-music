use std::sync::Arc;

use anyhow::Result;
use tokio::sync::{RwLock, broadcast};

use crate::internal_events::InternalEvent;
use crate::orchestrator::Orchestrator;

pub type SharedState = Arc<RwLock<AppState>>;

pub struct AppState {
    pub orchestrator: Orchestrator,
}

impl AppState {
    pub fn new(internal_event_tx: Arc<broadcast::Sender<InternalEvent>>) -> Result<Self> {
        Ok(AppState {
            orchestrator: Orchestrator::new(internal_event_tx)?,
        })
    }
}

pub fn new_shared_state(
    internal_event_tx: Arc<broadcast::Sender<InternalEvent>>,
) -> Result<SharedState> {
    Ok(Arc::new(RwLock::new(AppState {
        orchestrator: Orchestrator::new(internal_event_tx)?,
    })))
}
