use anyhow::Result;

use crate::collection::music::MusicItemId;
use crate::collection::folders::{FolderId, FolderDescription, FolderType};
use crate::collection::pictures::PictureId;

pub trait FoldersDbApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn FoldersDbApi>;
    fn get_root_folder(&self) -> FolderId;
    fn get_folder_parent(&self, folder_id: FolderId) -> Result<FolderId>;
    fn get_folder_name(&self, folder_id: FolderId) -> Result<String>;
    fn get_folder_description(&self, folder_id: FolderId) -> Result<FolderDescription>;
    fn find_or_add_folder(&self, parent_id: FolderId, folder_name: &str, folder_type: FolderType) -> Result<FolderId>;
    fn set_folder_name(&self, folder_id: FolderId, name: &str) -> Result<()>;
    fn set_folder_type(&self, folder_id: FolderId, folder_type: FolderType) -> Result<()>;
    fn set_folder_cover(&self, folder_id: FolderId, picture_id: PictureId) -> Result<()>;
    fn get_folders_in_folder(&self, folder_id: FolderId) -> Result<Vec<FolderDescription>>;
    fn get_music_items_in_folder(&self, folder_id: FolderId) -> Result<Vec<MusicItemId>>;
}
