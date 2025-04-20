use std::path::Path;

use anyhow::Result;
use rusqlite::params;

use crate::database::sqlite::utils::{DatabaseUtils, ProtobufExporter, ProtobufImporter};
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

    pub fn import(&self, base_path: &Path) -> Result<()> {
        let db_context = self.db_utils.lock();

        let mut importer = ProtobufImporter::create(base_path, "picture_items.pb")?;
        while let Some(row) = importer.read_next_row::<crate::proto::collection::PictureItemsRow>()? {
            db_context.connection().execute(
                "INSERT INTO picture_items (id, extension, folder_id) VALUES (?1, ?2, ?3)",
                params![row.picture_item_id, row.extension, row.folder_id],
            )?;
        }

        Ok(())
    }

    pub fn export(&self, base_path: &Path) -> Result<()> {
        let db_context = self.db_utils.lock();
        let mut exporter = ProtobufExporter::create(base_path, "picture_items.pb")?;
        let mut stmt = db_context.connection().prepare("SELECT id, extension, folder_id FROM picture_items")?;
        let rows = stmt.query_map([], |row| {
            let mut picture_row = crate::proto::collection::PictureItemsRow::new();
            picture_row.picture_item_id = row.get::<_, i64>(0)?;
            picture_row.extension = row.get::<_, String>(1)?;
            picture_row.folder_id = row.get::<_, i64>(2)?;
            Ok(picture_row)
        })?;
        for row in rows {
            exporter.write_row(&row?)?;
        }
        Ok(())
    }
}

impl PicturesDbApi for PicturesDb {
    fn clone_api(&self) -> Box<dyn PicturesDbApi> {
        return Box::new(PicturesDb::new(self.db_utils.clone()));
    }

    fn add_picture_item(&self, extension: &str, folder_id: FolderId) -> Result<PictureId> {
        let context = self.db_utils.lock();
        let query = "INSERT INTO picture_items (extension, folder_id) VALUES (?1, ?2)";
        context.connection().execute(&query, params![extension, folder_id])?;
        Ok(context.connection().last_insert_rowid())
    }

    fn delete_picture_item(&self, picture_id: PictureId) -> Result<()> {
        let mut context = self.db_utils.lock();
        let query = "DELETE FROM picture_items WHERE id = ?1";
        context.connection().execute(&query, params![picture_id])?;
        context.on_folders_updated();
        Ok(())
    }

    fn get_picture_extension(&self, picture_id: PictureId) -> Result<String> {
        let context = self.db_utils.lock();
        context.get_field_value(picture_id, "picture_items", "extension")
    }
    
    fn get_pictures_in_folder(&self, folder_id: FolderId) -> Result<Vec<PictureId>> {
        self.db_utils.lock().get_fields_list_by_field_i64_value("picture_items", "id", "folder_id", folder_id)
    }
}
