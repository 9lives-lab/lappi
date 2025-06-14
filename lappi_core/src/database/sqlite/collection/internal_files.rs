use std::path::Path;

use anyhow::Result;
use rusqlite::params;

use crate::collection::internal_files::database_api::InternalFilesDbApi;
use crate::collection::internal_files::{InternalFileId, InternalPath};
use crate::database::sqlite::utils::{DatabaseUtils, ProtobufExporter, ProtobufImporter};

pub struct InternalFilesDb {
    db_utils: DatabaseUtils,
}

impl InternalFilesDb {
    pub fn new(db_utils: DatabaseUtils) -> Self {
        Self {
            db_utils,
        }
    }

    pub fn import(&self, base_path: &Path) -> Result<()> {
        let db_context = self.db_utils.lock();

        let mut importer = ProtobufImporter::create(base_path, "internal_files.pb")?;
        while let Some(row) = importer.read_next_row::<crate::proto::collection::InternalFilesRow>()? {
            db_context.connection().execute(
                "INSERT INTO internal_files (id, internal_path) VALUES (?1, ?2)",
                params![row.file_id, row.internal_path],
            )?;
        }

        Ok(())
    }

    pub fn export(&self, base_path: &Path) -> Result<()> {
        let db_context = self.db_utils.lock();
        let mut exporter = ProtobufExporter::create(base_path, "internal_files.pb")?;
        let mut stmt = db_context.connection().prepare("SELECT id, internal_path FROM internal_files")?;
        let rows = stmt.query_map([], |row| {
            let mut picture_row = crate::proto::collection::InternalFilesRow::new();
            picture_row.file_id = row.get::<_, i64>(0)?;
            picture_row.internal_path = row.get::<_, String>(1)?;
            Ok(picture_row)
        })?;
        for row in rows {
            exporter.write_row(&row?)?;
        }
        Ok(())
    }
}

impl InternalFilesDbApi for InternalFilesDb {
    fn clone_api(&self) -> Box<dyn InternalFilesDbApi> {
        return Box::new(InternalFilesDb::new(self.db_utils.clone()));
    }

    fn add_file_path(&self, path: &InternalPath) -> Result<InternalFileId> {
        let db_context = self.db_utils.lock();
        let mut stmt = db_context.connection().prepare("INSERT INTO internal_files (internal_path) VALUES (?1)")?;
        stmt.execute([path.as_str()])?;
        Ok(db_context.connection().last_insert_rowid())
    }

    fn get_file_path(&self, file_id: InternalFileId) -> Result<InternalPath> {
        let db_context = self.db_utils.lock();
        let mut stmt = db_context.connection().prepare("SELECT internal_path FROM internal_files WHERE id = ?1")?;
        let row = stmt.query_row([file_id], |row| {
            row.get::<_, String>(0)
        })?;
        Ok(InternalPath::from_string(row))
    }
}
