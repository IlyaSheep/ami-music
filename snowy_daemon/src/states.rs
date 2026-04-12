use std::sync::Arc;

use anyhow::Result;
use snowy_core::library::Library;
use tokio::sync::Mutex;
use tokio::sync::broadcast;

use crate::internal_events::InternalEvent;
use crate::orchestrator::Orchestrator;

pub struct AppState {
    pub orchestrator: Arc<Mutex<Orchestrator>>,
    pub library: Library,
}

impl AppState {
    pub fn new(internal_event_tx: Arc<broadcast::Sender<InternalEvent>>) -> Result<Self> {
        Ok(AppState {
            orchestrator: Arc::new(Mutex::new(Orchestrator::new(internal_event_tx)?)),
            library: Library::default(),
        })
    }
}
