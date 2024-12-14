use crate::collection::folders::FolderId;
use crate::database::api::DbResult;
use crate::collection::music::MusicItemId;

use super::types::Tag;

pub trait TagsDbApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn TagsDbApi>;

    fn set_add_item_tag(&self, item_id: MusicItemId, tag_name: &str, tag_value: &str) -> DbResult<()>;
    fn get_item_tag(&self, item_id: MusicItemId, tag_name: &str) -> DbResult<Option<Tag>>;
    fn get_item_tags(&self, item_id: MusicItemId) -> DbResult<Vec<Tag>>;
    fn delete_item_tag(&self, item_id: MusicItemId, tag_name: &str) -> DbResult<()>;
    
    fn set_add_folder_tag(&self, folder_id: FolderId, tag_name: &str, tag_value: &str) -> DbResult<()>;
    fn get_folder_tag(&self, folder_id: FolderId, tag_name: &str) -> DbResult<Option<Tag>>;
    fn get_folder_tags(&self, folder_id: FolderId) -> DbResult<Vec<Tag>>;
    fn delete_folder_tag(&self, folder_id: FolderId, tag_name: &str) -> DbResult<()>;
}
