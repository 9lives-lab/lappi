use std::path::Path;

use anyhow::Result;
use rusqlite::params;

use crate::database::sqlite::utils::parse_enum;
use crate::database::sqlite::utils::parse_pb_enum;
use crate::database::sqlite::utils::{DatabaseUtils, ProtobufExporter, ProtobufImporter};
use crate::collection::folders::FolderId;
use crate::collection::pictures::{PictureDesc, PictureId, PictureType};
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
            db_context.connection().execute("INSERT INTO picture_items (id, folder_id, internal_file_id, picture_type) VALUES (?1, ?2, ?3, ?4)", params![
                row.id, 
                row.folder_id,
                row.internal_file_id,
                row.picture_type.value()
            ])?;
        }

        Ok(())
    }

    pub fn export(&self, base_path: &Path) -> Result<()> {
        let db_context = self.db_utils.lock();
        let mut exporter = ProtobufExporter::create(base_path, "picture_items.pb")?;
        let mut stmt = db_context.connection().prepare("SELECT id, folder_id, internal_file_id, picture_type FROM picture_items")?;
        let rows = stmt.query_map([], |row| {
            let mut picture_row = crate::proto::collection::PictureItemsRow::new();
            picture_row.id = row.get(0)?;
            picture_row.folder_id = row.get(1)?;
            picture_row.internal_file_id = row.get(2)?;
            picture_row.picture_type = parse_pb_enum::<crate::proto::collection::PictureType>(row.get::<_, i32>(3)?)?;
            Ok(picture_row)
        })?;
        exporter.write_rows(rows)
    }
}

impl PicturesDbApi for PicturesDb {
    fn clone_api(&self) -> Box<dyn PicturesDbApi> {
        return Box::new(PicturesDb::new(self.db_utils.clone()));
    }

    fn add_picture_item(&self, descriptor: &PictureDesc) -> Result<PictureId> {
        let context = self.db_utils.lock();
        context.connection().execute("INSERT INTO picture_items (folder_id, internal_file_id, picture_type) VALUES (?1, ?2, ?3)", params![
            descriptor.folder_id, 
            descriptor.internal_file_id,
            descriptor.picture_type as i32
        ])?;
        Ok(context.connection().last_insert_rowid())
    }

    fn update_picture_item(&self, descriptor: &PictureDesc) -> Result<()> {
        let context = self.db_utils.lock();
        context.connection().execute("UPDATE picture_items SET folder_id = ?1, internal_file_id = ?2, picture_type = ?3 WHERE id = ?4", params![
            descriptor.folder_id,
            descriptor.internal_file_id,
            descriptor.picture_type as i32,
            descriptor.picture_id
        ])?;
        Ok(())
    }

    fn delete_picture_item(&self, picture_id: PictureId) -> Result<()> {
        let mut context = self.db_utils.lock();
        let query = "DELETE FROM picture_items WHERE id = ?1";
        context.connection().execute(&query, params![picture_id])?;
        context.on_folders_updated();
        Ok(())
    }

    fn get_picture_descriptor(&self, picture_id: PictureId) -> Result<PictureDesc> {
        let context = self.db_utils.lock();
        let mut stmt = context.connection().prepare("SELECT folder_id, internal_file_id, picture_type FROM picture_items WHERE id = ?1")?;
        let row = stmt.query_row(params![picture_id], |row| {
            Ok(PictureDesc {
                picture_id,
                folder_id: row.get(0)?,
                internal_file_id: row.get(1)?,
                picture_type: parse_enum::<PictureType>(row.get(2)?)?,
            })
        })?;
        Ok(row)
    }
    
    fn get_pictures_in_folder(&self, folder_id: FolderId) -> Result<Vec<PictureDesc>> {
        let context = self.db_utils.lock();
        let mut stmt = context.connection().prepare("SELECT id, internal_file_id, picture_type FROM picture_items WHERE folder_id = ?1")?;
        let rows = stmt.query_map(params![folder_id], |row| {
            Ok(PictureDesc {
                picture_id: row.get(0)?,
                folder_id,
                internal_file_id: row.get(1)?,
                picture_type: parse_enum::<PictureType>(row.get(2)?)?,
            })
        })?;
        Ok(rows.collect::<Result<Vec<_>, _>>()?)
    }
}
