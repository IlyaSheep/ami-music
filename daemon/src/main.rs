use std::sync::Arc;

use ami_core::config::Config;
use ami_daemon::{
    internal_events::InternalEvent, logging::setup_logger, orchestrator::Orchestrator, services,
    states::new_shared_state, websockets::WebSocketService,
};
use anyhow::Result;
use tokio::{net::TcpListener, sync::broadcast};

// How many messages the broadcast channel can buffer
const CHANNEL_CAPACITY: usize = 32;
const DAEMON_ADDR: &str = "0.0.0.0:7878";

#[tokio::main]
async fn main() -> Result<()> {
    setup_logger()?;
    log::debug!("Daemon starting...");

    let (internal_event_tx, _) = broadcast::channel::<InternalEvent>(CHANNEL_CAPACITY);
    let internal_event_tx = Arc::new(internal_event_tx);

    let shared_state = new_shared_state(Arc::clone(&internal_event_tx))?;
    let config = Config::load()?;

    shared_state
        .write()
        .await
        .orchestrator
        .library
        .load(config.library);

    services::run_thumbnail_service()?;

    let tx = Arc::clone(&internal_event_tx);
    let player = Arc::clone(&shared_state.read().await.orchestrator.playback.player);
    tokio::spawn(async move { Orchestrator::send_player_position(player, tx).await });

    let listener = TcpListener::bind(DAEMON_ADDR).await?;
    log::debug!("Server listening on {DAEMON_ADDR}");

    // A broadcast channel: one sender, many receivers (one per client)
    let (connection_tx, _) = broadcast::channel::<String>(CHANNEL_CAPACITY);
    let connection_tx = Arc::new(connection_tx); // Share the sender across tasks

    let ws_service =
        WebSocketService::new(listener, connection_tx, internal_event_tx, shared_state);
    ws_service.start().await
}
