use anyhow::Result;

use crate::collection::folders::FolderId;
use crate::collection::music::MusicItemId;
use super::types::Tag;
use super::TagValue;

pub trait TagsDbApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn TagsDbApi>;

    fn set_add_item_tag(&self, item_id: MusicItemId, tag_name: &str, tag_value: &TagValue) -> Result<()>;
    fn get_item_tag(&self, item_id: MusicItemId, tag_name: &str) -> Result<Option<Tag>>;
    fn get_item_tags(&self, item_id: MusicItemId) -> Result<Vec<Tag>>;
    fn delete_item_tag(&self, item_id: MusicItemId, tag_name: &str) -> Result<()>;
    
    fn set_add_folder_tag(&self, folder_id: FolderId, tag_name: &str, tag_value: &TagValue) -> Result<()>;
    fn get_folder_tag(&self, folder_id: FolderId, tag_name: &str) -> Result<Option<Tag>>;
    fn get_folder_tags(&self, folder_id: FolderId) -> Result<Vec<Tag>>;
    fn delete_folder_tag(&self, folder_id: FolderId, tag_name: &str) -> Result<()>;
}
