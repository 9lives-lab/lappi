use crate::database_api::DbResult;
use crate::playlists::types::{PlaylistDesc, PlaylistId};

pub trait DatabaseApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn DatabaseApi>;
    fn create_classic_playlist(&self, name: &str) -> DbResult<PlaylistId>;
    fn get_classic_playlists(&self) -> DbResult<Vec<PlaylistDesc>>;
}
