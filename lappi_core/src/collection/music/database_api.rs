use crate::database::api::DbResult;
use crate::collection::folders::FolderId;
use super::{ExternalSrcFileDesc, MusicItemDescription, MusicItemId, Tag};

pub trait MusicDbApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn MusicDbApi>;
    // Music items
    fn add_music_item(&self, name: &str, folder_id: FolderId) -> MusicItemId;
    fn get_music_item_description(&self, music_id: MusicItemId) -> DbResult<MusicItemDescription>;
    fn get_all_music_items(&self) -> DbResult<Vec<MusicItemId>>;
    fn get_music_item_folder(&self, item_id: MusicItemId) -> DbResult<FolderId>;
    // Tags
    fn add_tag(&self, item_id: MusicItemId, name: &str, value: &str) -> DbResult<()>;
    fn get_tag(&self, item_id: MusicItemId, key: &str) -> DbResult<Option<Tag>>;
    fn get_tags(&self, item_id: MusicItemId) -> DbResult<Vec<Tag>>;
    // Song files
    fn add_external_src_file(&self, item_id: MusicItemId, path: &str) -> DbResult<()>;
    fn get_external_src_files(&self, item_id: MusicItemId) -> DbResult<Vec<ExternalSrcFileDesc>>;
}
