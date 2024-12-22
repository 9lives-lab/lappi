use crate::collection::music::MusicItemId;
use crate::collection::folders::{FolderId, FolderDescription, FolderType};
use crate::collection::pictures::PictureId;
use crate::database::api::DbResult;

pub trait FoldersDbApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn FoldersDbApi>;
    fn get_root_folder(&self) -> FolderId;
    fn get_folder_parent(&self, folder_id: FolderId) -> DbResult<FolderId>;
    fn get_folder_name(&self, folder_id: FolderId) -> DbResult<String>;
    fn get_folder_description(&self, folder_id: FolderId) -> DbResult<FolderDescription>;
    fn find_or_add_folder(&self, parent_id: FolderId, folder_name: &str, folder_type: FolderType) -> DbResult<FolderId>;
    fn set_folder_name(&self, folder_id: FolderId, name: &str) -> DbResult<()>;
    fn set_folder_type(&self, folder_id: FolderId, folder_type: FolderType) -> DbResult<()>;
    fn set_folder_cover(&self, folder_id: FolderId, picture_id: PictureId) -> DbResult<()>;
    fn get_folders_in_folder(&self, folder_id: FolderId) -> DbResult<Vec<FolderDescription>>;
    fn get_music_items_in_folder(&self, folder_id: FolderId) -> DbResult<Vec<MusicItemId>>;
}
