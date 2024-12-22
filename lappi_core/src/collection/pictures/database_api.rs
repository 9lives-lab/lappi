use crate::database::api::DbResult;
use crate::collection::folders::FolderId;

use super::PictureId;

pub trait PicturesDbApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn PicturesDbApi>;
    fn add_picture_item(&self, extension: &str, folder_id: FolderId) -> DbResult<PictureId>;
    fn delete_picture_item(&self, picture_id: PictureId) -> DbResult<()>;
    fn get_picture_extension(&self, picture_id: PictureId) -> DbResult<String>;
    fn get_pictures_in_folder(&self, folder_id: FolderId) -> DbResult<Vec<PictureId>>;
}
