use std::{fs::File, path::Path};

use anyhow::Result;
use rodio::{Decoder, MixerDeviceSink, Player};

use crate::player::playback_status::PlaybackStatus;

#[cfg(test)]
pub mod tests;

pub mod mpris;
pub mod playback_status;

/// Performs player-related functionalities.
pub struct Playback {
    player: Player,
    sink: MixerDeviceSink,
}

impl Playback {
    pub fn new() -> Result<Self> {
        let sink = rodio::DeviceSinkBuilder::open_default_sink()?;
        Ok(Playback {
            player: rodio::Player::connect_new(sink.mixer()),
            sink,
        })
    }

    /// Append audio source from path to the sink.
    pub fn load_track(&self, audio_path: &Path) -> Result<()> {
        let source = Decoder::try_from(File::open(audio_path)?)?;
        self.player.append(source);

        Ok(())
    }

    pub fn play(&self) {
        self.player.play();
    }

    pub fn pause(&self) {
        self.player.pause();
    }

    pub fn toggle_play(&self) {
        if self.player.is_paused() {
            self.player.play();
        } else {
            self.player.play();
        }
    }

    /// Returns f64 as volume
    pub fn volume(&self) -> f32 {
        self.player.volume()
    }

    pub fn set_volume(&self, value: f32) {
        self.player.set_volume(value);
    }

    pub fn playback_speed(&self) -> f32 {
        self.player.speed()
    }

    pub fn playback_status(&self) -> PlaybackStatus {
        if self.player.empty() {
            PlaybackStatus::Stopped
        } else if self.player.is_paused() {
            PlaybackStatus::Paused
        } else {
            PlaybackStatus::Playing
        }
    }
}
