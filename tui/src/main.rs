use std::{path::PathBuf, sync::Arc, time::SystemTime};

use ami_core::library::Library;
use ami_daemon::{
    commands::{Command, LibraryCommand},
    events::ServerEvent,
};
use color_eyre::eyre::Result;
use futures::{SinkExt, StreamExt};
use tokio::{
    net::TcpStream,
    sync::{
        Mutex,
        mpsc::{self, UnboundedReceiver},
    },
};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::{app::App, state::AppStates};

pub mod app;
pub mod event;
pub mod state;
pub mod ui;

const URL: &str = "ws://0.0.0.0:7878";

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    setup_logger()?;

    let terminal = ratatui::init();

    let states = Arc::new(Mutex::new(AppStates::default()));

    let (ws, _) = tokio_tungstenite::connect_async(URL).await?;
    println!("Connected to {URL}");

    let (tx, rx) = mpsc::unbounded_channel::<Command>();

    let app = App::new(states.clone(), tx);

    tokio::spawn(connect(ws, rx, states));
    let result = app.run(terminal).await;
    ratatui::restore();

    result
}

async fn connect(
    ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
    mut rx: UnboundedReceiver<Command>,
    states: Arc<Mutex<AppStates>>,
) -> Result<()> {
    let (mut ws_sink, mut ws_stream) = ws.split();

    // Initial fetch command
    let json = serde_json::to_string(&Command::Library(LibraryCommand::Fetch))?;
    ws_sink
        .send(tokio_tungstenite::tungstenite::Message::Text(json.into()))
        .await?;

    loop {
        tokio::select! {
            cmd_opt = rx.recv() => {
                match cmd_opt {
                    Some(cmd) => {
                        let json = serde_json::to_string(&cmd).unwrap();
                        ws_sink.send(tokio_tungstenite::tungstenite::Message::Text(json.into())).await?;
                    }
                    None => break, // Channel closed
                }
            }

            // Mutate States
            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(tokio_tungstenite::tungstenite::Message::Text(text))) => {
                        if let Ok(event) = serde_json::from_str::<ServerEvent>(&text) {
                            match event {
                                ServerEvent::SendLibrary(tracks) => {
                                    let mut states = states.lock().await;
                                    states.library_snapshot = Library { tracks };
                                },
                                ServerEvent::SendQueue(queue) => {
                                    let mut states = states.lock().await;
                                    states.queue_snapshot = queue;
                                },
                                ServerEvent::SendPlayerSnapshot(snapshot) => {
                                    let mut states = states.lock().await;
                                    states.player_snapshot = snapshot;
                                },
                            }
                        }
                    },
                    Some(Err(e)) => {
                        println!("WebSocket error: {}", e);
                        break;
                    }
                    None => break, // Connection closed
                    _ => {} // Ignore binary, pings, etc.
                }
            }
        }
    }

    Ok(())
}

pub fn setup_logger() -> Result<()> {
    let log_path = PathBuf::from("/home/lyns0/projects/personal/ami/tui.log");

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("lofty", log::LevelFilter::Error)
        .level_for("zbus", log::LevelFilter::Error)
        .level_for("tracing", log::LevelFilter::Error)
        .chain(fern::log_file(log_path)?)
        .apply()?;
    Ok(())
}
