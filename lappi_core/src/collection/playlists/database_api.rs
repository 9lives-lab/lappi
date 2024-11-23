use crate::database::api::DbResult;
use crate::collection::music::MusicItemId;
use super::types::{PlaylistDesc, PlaylistId, PlaylistItemId};

pub trait PlaylistsDbApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn PlaylistsDbApi>;
    fn create_playlist(&self, name: &str) -> DbResult<PlaylistId>;
    fn set_playlist_name(&self, id: PlaylistId, name: &str) -> DbResult<()>;
    fn delete_playlist(&self, id: PlaylistId) -> DbResult<()>;
    fn get_playlists(&self) -> DbResult<Vec<PlaylistDesc>>;
    fn get_playlist_description(&self, id: PlaylistId) -> DbResult<PlaylistDesc>;
    fn add_item_to_playlist(&self, playlist_id: PlaylistId, music_item_id: MusicItemId) -> DbResult<()>;
    fn get_playlist_items(&self, playlist_id: PlaylistId) -> DbResult<Vec<(PlaylistItemId, MusicItemId)>>;
    fn get_playlists_for_music_item(&self, music_item_id: MusicItemId) -> DbResult<Vec<PlaylistId>>;
}
