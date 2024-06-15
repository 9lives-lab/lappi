use serde::{Serialize, Deserialize};
use amina_core::events::Event;

use crate::database_api::{DbExporter, DbImporter, DbResult};
use crate::collection::types::{FolderId, ItemId, LyricsId, MusicItemId, PictureId};
use crate::collection::types::tags::Tag;
use crate::collection::music::MusicItemDescription;
use crate::collection::music::types::ExternalSrcFileDesc;
use crate::collection::folders::{FolderDescription, FolderType};
use crate::collection::lyrics::LyricsDescription;

#[derive(Default, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Event)]
#[key = "lappi.collection.database.OnItemsUpdated"]
pub struct OnItemsUpdated {
    pub items: Vec<ItemId>,
    pub tags_updated: bool,
}

pub trait DatabaseApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn DatabaseApi>;
    fn start_batch(&self);
    fn stop_batch(&self);

    // Folders
    fn get_root_folder(&self) -> FolderId;
    fn get_folder_parent(&self, folder_id: FolderId) -> DbResult<FolderId>;
    fn get_folder_name(&self, folder_id: FolderId) -> DbResult<String>;
    fn get_folder_description(&self, folder_id: FolderId) -> DbResult<FolderDescription>;
    fn find_or_add_folder(&self, parent_id: FolderId, folder_name: &str, folder_type: FolderType) -> DbResult<FolderId>;
    fn get_folders_in_folder(&self, folder_id: FolderId) -> DbResult<Vec<FolderDescription>>;

    // Music items
    fn add_music_item(&self, name: &str, folder_id: FolderId) -> MusicItemId;
    fn get_music_item_description(&self, music_id: MusicItemId) -> DbResult<MusicItemDescription>;
    fn get_all_music_items(&self) -> DbResult<Vec<MusicItemId>>;
    fn get_music_item_folder(&self, item_id: MusicItemId) -> DbResult<FolderId>;
    fn get_music_items_in_folder(&self, folder_id: FolderId) -> DbResult<Vec<MusicItemId>>;

    // Lyrics
    fn add_lyrics_item(&self, music_id: MusicItemId, lang_code: &str) -> DbResult<LyricsId>;
    fn get_lyrics_list(&self, music_id: MusicItemId) -> DbResult<Vec<LyricsDescription>>;

    // Pictures
    fn add_picture_item(&self, extension: &str, folder_id: FolderId) -> DbResult<PictureId>;
    fn get_picture_extension(&self, picture_id: PictureId) -> DbResult<String>;
    fn get_pictures_in_folder(&self, folder_id: FolderId) -> DbResult<Vec<PictureId>>;

    // Tags
    fn add_tag(&self, item_id: ItemId, name: &str, value: &str) -> DbResult<()>;
    fn get_tag(&self, item_id: ItemId, key: &str) -> DbResult<Option<Tag>>;
    fn get_tags(&self, item_id: ItemId) -> DbResult<Vec<Tag>>;

    // Song files
    fn add_external_src_file(&self, item_id: ItemId, path: &str) -> DbResult<()>;
    fn get_external_src_files(&self, item_id: ItemId) -> DbResult<Vec<ExternalSrcFileDesc>>;

    fn export(&self, exporter: Box<dyn DbExporter>) -> DbResult<()>;
    fn import(&self, importer: Box<dyn DbImporter>) -> DbResult<()>;
}
