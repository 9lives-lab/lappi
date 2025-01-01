use anyhow::Result;

use crate::collection::folders::FolderId;
use super::PictureId;

pub trait PicturesDbApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn PicturesDbApi>;
    fn add_picture_item(&self, extension: &str, folder_id: FolderId) -> Result<PictureId>;
    fn delete_picture_item(&self, picture_id: PictureId) -> Result<()>;
    fn get_picture_extension(&self, picture_id: PictureId) -> Result<String>;
    fn get_pictures_in_folder(&self, folder_id: FolderId) -> Result<Vec<PictureId>>;
}
