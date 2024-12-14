use num_traits::FromPrimitive;
use rusqlite::params;

use crate::collection::folders::FolderId;
use crate::collection::music::database_api::MusicDbApi;
use crate::collection::music::{MusicItemDescription, MusicItemId, MusicSourceFileId, SourceFileDesc, SourceType};
use crate::database::sqlite::utils::DatabaseUtils;
use crate::database::api::DbResult;

pub struct MusicDb {
    db_utils: DatabaseUtils,
}

impl MusicDb {
    pub fn new(db_utils: DatabaseUtils) -> Self {
        Self {
            db_utils,
        }
    }
}

impl MusicDbApi for MusicDb {
    fn clone_api(&self) -> Box<dyn MusicDbApi> {
        return Box::new(MusicDb::new(self.db_utils.clone()));
    }

    fn add_music_item(&self, name: &str, folder_id: FolderId) -> MusicItemId {
        let mut context = self.db_utils.lock();
        context.connection().execute(
            "INSERT INTO music_items (name, folder_id) VALUES (?1, ?2)",
            params![name, folder_id],
        ).unwrap();
        let item_id = context.connection().last_insert_rowid();
        context.on_folders_updated(); 
        item_id
    }

    fn set_item_name(&self, item_id: MusicItemId, name: &str) -> DbResult<()> {
        let mut context = self.db_utils.lock();
        context.set_field_value(item_id, "music_items", "name", name)?;
        context.on_folders_updated(); // Notify any observers of the change
        Ok(())
    }

    fn get_music_item_description(&self, music_id: MusicItemId) -> DbResult<MusicItemDescription> {
        let context = self.db_utils.lock();
        let description = context.connection().query_row(
            "SELECT name, folder_id FROM music_items WHERE id=(?1)",
            params![music_id],
            |row| {
                Ok(MusicItemDescription {
                    item_id: music_id,
                    name:        row.get:: < _, String>(0)?,
                    folder_id:   row.get:: < _, i64>(1)? as FolderId,
                })
            },
        )?;
    
        Ok(description) 
    }

    fn get_all_music_items(&self) -> DbResult<Vec<MusicItemId>> {
        self.db_utils.lock().get_rows_list("music_items")
    }

    fn get_music_item_folder(&self, item_id: MusicItemId) -> DbResult<FolderId> {
        self.db_utils.lock().get_field_value(item_id, "music_items","folder_id")
    }

    fn add_source_file(&self, item_id: MusicItemId, source_type: SourceType, path: &str) -> DbResult<()> {
        let mut context = self.db_utils.lock();
        context.connection().execute(
            "INSERT INTO music_src_files (music_item_id, path, source_type) VALUES (?1, ?2, ?3)",
            params![item_id, path, source_type as i32],
        )?;
        context.on_music_updated();
        Ok(())
    }

    fn delete_source_file(&self, source_id: MusicSourceFileId) -> DbResult<()> {
        let mut context = self.db_utils.lock();
        let conn = context.connection();
        conn.execute(
            "DELETE FROM music_src_files WHERE id = ?1",
            params![source_id],
        )?;
        context.on_music_updated();
        Ok(())
    }

    fn set_source_file_path(&self, source_id: MusicSourceFileId, path: &str) -> DbResult<()> {
        let mut context = self.db_utils.lock();
        context.set_field_value(source_id, "music_src_files", "path", path)?;
        context.on_music_updated();
        Ok(())
    }

    fn get_source_files(&self, item_id: MusicItemId) -> DbResult<Vec<SourceFileDesc>> {
        let context = self.db_utils.lock();
        let mut stmt = context.connection().prepare(
            "SELECT id, music_item_id, path, source_type FROM music_src_files WHERE music_item_id=(?1)"
        )?;
        let rows = stmt.query_map(
            params![item_id], |row| Ok(
                SourceFileDesc {
                    id: row.get::<_, i32>(0)? as MusicItemId,
                    music_item_id: row.get::<_, i32>(1)? as MusicItemId,
                    path: row.get::<_, String>(2)?,
                    source_type: SourceType::from_i32(row.get::<_, i32>(3)?).unwrap(),
                }
            )
        )?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }
}
