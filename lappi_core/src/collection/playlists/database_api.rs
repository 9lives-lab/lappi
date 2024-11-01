use crate::database::api::DbResult;
use super::types::{PlaylistDesc, PlaylistId};

pub trait PlaylistsDbApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn PlaylistsDbApi>;
    fn create_classic_playlist(&self, name: &str) -> DbResult<PlaylistId>;
    fn get_classic_playlists(&self) -> DbResult<Vec<PlaylistDesc>>;
}
