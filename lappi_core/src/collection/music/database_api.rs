use crate::database::api::DbResult;
use crate::collection::folders::FolderId;
use super::{MusicItemDescription, MusicItemId, MusicSourceFileId, SourceFileDesc, SourceType, Tag};

pub trait MusicDbApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn MusicDbApi>;
    // Music items
    fn add_music_item(&self, name: &str, folder_id: FolderId) -> MusicItemId;
    fn set_item_name(&self, item_id: MusicItemId, name: &str) -> DbResult<()>;
    fn get_music_item_description(&self, music_id: MusicItemId) -> DbResult<MusicItemDescription>;
    fn get_all_music_items(&self) -> DbResult<Vec<MusicItemId>>;
    fn get_music_item_folder(&self, item_id: MusicItemId) -> DbResult<FolderId>;
    // Tags
    fn add_tag(&self, item_id: MusicItemId, name: &str, value: &str) -> DbResult<()>;
    fn get_tag(&self, item_id: MusicItemId, key: &str) -> DbResult<Option<Tag>>;
    fn get_tags(&self, item_id: MusicItemId) -> DbResult<Vec<Tag>>;
    // Song files
    fn add_source_file(&self, item_id: MusicItemId, source_type: SourceType, path: &str) -> DbResult<()>;
    fn delete_source_file(&self, source_id: MusicSourceFileId) -> DbResult<()>;
    fn set_source_file_path(&self, source_id: MusicSourceFileId, path: &str) -> DbResult<()>;
    fn get_source_files(&self, item_id: MusicItemId) -> DbResult<Vec<SourceFileDesc>>;
}
