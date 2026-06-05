use crate::playlist_manager::playlist::Playlist;

pub mod playlist;

#[derive(Debug, Default)]
pub struct PlaylistManager {
    pub playlists: Vec<Playlist>,
}
