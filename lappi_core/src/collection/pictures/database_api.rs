use anyhow::Result;

use crate::collection::{folders::FolderId, pictures::PictureDesc};
use super::PictureId;

pub trait PicturesDbApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn PicturesDbApi>;
    fn add_picture_item(&self, descriptor: &PictureDesc) -> Result<PictureId>;
    fn update_picture_item(&self, descriptor: &PictureDesc) -> Result<()>;
    fn delete_picture_item(&self, picture_id: PictureId) -> Result<()>;
    fn get_picture_descriptor(&self, picture_id: PictureId) -> Result<PictureDesc>;
    fn get_pictures_in_folder(&self, folder_id: FolderId) -> Result<Vec<PictureDesc>>;
}
