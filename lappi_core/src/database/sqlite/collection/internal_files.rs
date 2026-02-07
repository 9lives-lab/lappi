use anyhow::Result;
use camino::Utf8Path;
use rusqlite::params;

use crate::collection::internal_files::database_api::InternalFilesDbApi;
use crate::collection::internal_files::{FileHash, InternalFileId, InternalPath};
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

    pub fn import(&self, base_path: &Utf8Path) -> Result<()> {
        let db_context = self.db_utils.lock();

        let mut importer = ProtobufImporter::create(&base_path.join("internal_files.pb"))?;
        while let Some(row) = importer.read_next_row::<crate::proto::collection::InternalFilesRow>()? {
            db_context.connection().execute(
                "INSERT INTO internal_files (id, internal_path, hash) VALUES (?1, ?2, ?3)",
                params![row.file_id, row.internal_path, row.hash],
            )?;
        }

        Ok(())
    }

    pub fn export(&self, base_path: &Utf8Path) -> Result<()> {
        let db_context = self.db_utils.lock();
        let mut exporter = ProtobufExporter::create(base_path, "internal_files.pb")?;
        let mut stmt = db_context.connection().prepare("SELECT id, internal_path, hash FROM internal_files")?;
        let rows = stmt.query_map([], |row| {
            let mut internal_files_row = crate::proto::collection::InternalFilesRow::new();
            internal_files_row.file_id = row.get::<_, i64>(0)?;
            internal_files_row.internal_path = row.get::<_, String>(1)?;
            internal_files_row.hash = row.get::<_, Vec<u8>>(2)?;

            Ok(internal_files_row)
        })?;
        exporter.write_rows(rows)?;
        exporter.generate_hash()?;
        Ok(())
    }
}

impl InternalFilesDbApi for InternalFilesDb {
    fn clone_api(&self) -> Box<dyn InternalFilesDbApi> {
        return Box::new(InternalFilesDb::new(self.db_utils.clone()));
    }

    fn add_file_path(&self, path: &InternalPath) -> Result<InternalFileId> {
        let db_context = self.db_utils.lock();
        let mut stmt = db_context.connection().prepare("INSERT INTO internal_files (internal_path, hash) VALUES (?1, ?2)")?;
        let hash = Vec::<u8>::new();
        stmt.execute(params![path.as_str(), hash.as_slice()])?;
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

    fn get_file_hash(&self, file_id: InternalFileId) -> Result<FileHash> {
        let db_context = self.db_utils.lock();
        let hash_bytes = db_context.get_field_value::<Vec<u8>>(file_id, "internal_files", "hash");
        Ok(FileHash::from(hash_bytes?))
    }

    fn set_file_path(&self, file_id: InternalFileId, path: &InternalPath) -> Result<()> {
        let db_context = self.db_utils.lock();
        db_context.set_field_value(file_id, "internal_files", "internal_path", path.as_str())
    }

    fn set_file_hash(&self, file_id: InternalFileId, hash: &FileHash) -> Result<()> {
        let db_context = self.db_utils.lock();
        db_context.set_field_value(file_id, "internal_files", "hash", hash.bytes.as_slice())
    }

    fn delete_file(&self, file_id: InternalFileId) -> Result<()> {
        let db_context = self.db_utils.lock();
        let mut stmt = db_context.connection().prepare("DELETE FROM internal_files WHERE id = ?1")?;
        stmt.execute([file_id])?;
        Ok(())
    }

    fn get_all_files(&self) -> Result<Vec<InternalFileId>> {
        let db_context = self.db_utils.lock();
        db_context.get_rows_list("internal_files")
    }
}
