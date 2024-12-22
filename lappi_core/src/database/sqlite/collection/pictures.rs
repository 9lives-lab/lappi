use rusqlite::params;

use crate::database::api::DbResult;
use crate::database::sqlite::utils::DatabaseUtils;
use crate::collection::folders::FolderId;
use crate::collection::pictures::PictureId;
use crate::collection::pictures::database_api::PicturesDbApi;

pub struct PicturesDb {
    db_utils: DatabaseUtils,
}

impl PicturesDb {
    pub fn new(db_utils: DatabaseUtils) -> Self {
        Self {
            db_utils,
        }
    }
}

impl PicturesDbApi for PicturesDb {
    fn clone_api(&self) -> Box<dyn PicturesDbApi> {
        return Box::new(PicturesDb::new(self.db_utils.clone()));
    }

    fn add_picture_item(&self, extension: &str, folder_id: FolderId) -> DbResult<PictureId> {
        let context = self.db_utils.lock();
        let query = "INSERT INTO picture_items (extension, folder_id) VALUES (?1, ?2)";
        context.connection().execute(&query, params![extension, folder_id])?;
        Ok(context.connection().last_insert_rowid())
    }

    fn delete_picture_item(&self, picture_id: PictureId) -> DbResult<()> {
        let mut context = self.db_utils.lock();
        let query = "DELETE FROM picture_items WHERE id = ?1";
        context.connection().execute(&query, params![picture_id])?;
        context.on_folders_updated();
        Ok(())
    }

    fn get_picture_extension(&self, picture_id: PictureId) -> DbResult<String> {
        let context = self.db_utils.lock();
        context.get_field_value(picture_id, "picture_items", "extension")
    }
    
    fn get_pictures_in_folder(&self, folder_id: FolderId) -> DbResult<Vec<PictureId>> {
        self.db_utils.lock().get_fields_list_by_field_i64_value("picture_items", "id", "folder_id", folder_id)
    }
}
