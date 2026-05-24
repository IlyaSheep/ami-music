use std::sync::Arc;

use ami_core::{
    player::playback_status::PlaybackStatus,
    queue::loop_mode::{self, LoopMode},
};
use anyhow::Result;
use mpris_server::Server;
use tokio::sync::{RwLock, mpsc::UnboundedSender};

use crate::{
    app::{MprisServer, SharedState},
    commands::Command,
};

pub mod player_interface;
pub mod root_interface;

pub const BUS_NAME: &str = "AmiMusic";

pub struct Mpris {
    pub shared_state: SharedState,
    pub command_tx: UnboundedSender<Command>,
}

impl Mpris {
    pub fn new(shared_state: SharedState, command_tx: UnboundedSender<Command>) -> Mpris {
        Mpris {
            shared_state,
            command_tx,
        }
    }

    pub async fn start(self) -> Result<MprisServer> {
        Ok(Arc::new(RwLock::new(
            Server::new(&format!("org.mpris.MediaPlayer2.{}", BUS_NAME), self).await?,
        )))
    }

    pub fn match_playback_status(playback_status: PlaybackStatus) -> mpris_server::PlaybackStatus {
        match playback_status {
            PlaybackStatus::Paused => mpris_server::PlaybackStatus::Paused,
            PlaybackStatus::Playing => mpris_server::PlaybackStatus::Playing,
            PlaybackStatus::Stopped => mpris_server::PlaybackStatus::Stopped,
        }
    }

    pub fn match_loop_status(loop_mode: LoopMode) -> mpris_server::LoopStatus {
        match loop_mode {
            loop_mode::LoopMode::None => mpris_server::LoopStatus::None,
            loop_mode::LoopMode::Queue => mpris_server::LoopStatus::Playlist,
            loop_mode::LoopMode::Track => mpris_server::LoopStatus::Track,
        }
    }
}
