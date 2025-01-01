use anyhow::Result;

use crate::collection::folders::FolderId;
use super::{MusicItemDescription, MusicItemId, MusicSourceFileId, SourceFileDesc, SourceType};

pub trait MusicDbApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn MusicDbApi>;
    // Music items
    fn add_music_item(&self, name: &str, folder_id: FolderId) -> MusicItemId;
    fn set_item_name(&self, item_id: MusicItemId, name: &str) -> Result<()>;
    fn get_music_item_description(&self, music_id: MusicItemId) -> Result<MusicItemDescription>;
    fn get_all_music_items(&self) -> Result<Vec<MusicItemId>>;
    fn get_music_item_folder(&self, item_id: MusicItemId) -> Result<FolderId>;
    // Song files
    fn add_source_file(&self, item_id: MusicItemId, source_type: SourceType, path: &str) -> Result<()>;
    fn delete_source_file(&self, source_id: MusicSourceFileId) -> Result<()>;
    fn set_source_file_path(&self, source_id: MusicSourceFileId, path: &str) -> Result<()>;
    fn get_source_files(&self, item_id: MusicItemId) -> Result<Vec<SourceFileDesc>>;
}
