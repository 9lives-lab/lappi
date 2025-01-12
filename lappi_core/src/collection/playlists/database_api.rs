use anyhow::Result;

use crate::collection::music::MusicItemId;
use crate::collection::pictures::PictureId;
use super::types::{PlaylistDesc, PlaylistId, PlaylistItemId};

pub trait PlaylistsDbApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn PlaylistsDbApi>;
    fn create_playlist(&self, name: &str) -> Result<PlaylistId>;
    fn set_playlist_name(&self, id: PlaylistId, name: &str) -> Result<()>;
    fn set_playlist_cover(&self, id: PlaylistId, picture_id: Option<PictureId>) -> Result<()>;
    fn delete_playlist(&self, id: PlaylistId) -> Result<()>;
    fn get_playlists(&self) -> Result<Vec<PlaylistDesc>>;
    fn get_playlist_description(&self, id: PlaylistId) -> Result<PlaylistDesc>;
    fn add_item_to_playlist(&self, playlist_id: PlaylistId, music_item_id: MusicItemId) -> Result<()>;
    fn delete_item_from_playlist(&self, playlist_id: PlaylistId, music_item_id: MusicItemId) -> Result<()>;
    fn get_playlist_items(&self, playlist_id: PlaylistId) -> Result<Vec<(PlaylistItemId, MusicItemId)>>;
    fn get_playlists_for_music_item(&self, music_item_id: MusicItemId) -> Result<Vec<PlaylistId>>;
}
