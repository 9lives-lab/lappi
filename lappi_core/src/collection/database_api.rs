use std::path::Path;

use anyhow::Result;
use serde::{Serialize, Deserialize};
use amina_core::events::Event;

use crate::collection::folders::database_api::FoldersDbApi;
use crate::collection::lyrics::database_api::LyricsDbApi;
use crate::collection::music::database_api::MusicDbApi;
use crate::collection::tags::database_api::TagsDbApi;
use crate::collection::pictures::database_api::PicturesDbApi;
use crate::collection::playlists::database_api::PlaylistsDbApi;

#[derive(Default, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Event)]
#[key = "lappi.collection.OnCollectionUpdated"]
pub struct OnCollectionUpdated {
    pub folders_updated: bool,
    pub music_updated: bool,
    pub plalists_updated: bool,
}

pub trait CollectionDbApi: Send + Sync {
    fn get_folders_api(&self) -> Box<dyn FoldersDbApi>;
    fn get_lyrics_api(&self) -> Box<dyn LyricsDbApi>;
    fn get_music_api(&self) -> Box<dyn MusicDbApi>;
    fn get_tags_api(&self) -> Box<dyn TagsDbApi>;
    fn get_pictures_api(&self) -> Box<dyn PicturesDbApi>;
    fn get_playlist(&self) -> Box<dyn PlaylistsDbApi>;

    fn start_batch(&self);
    fn stop_batch(&self);
 
    fn export(&self, base_path: &Path) -> Result<()>;
    fn import(&self, base_path: &Path) -> Result<()>;
}