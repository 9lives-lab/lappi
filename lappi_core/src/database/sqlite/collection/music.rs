use std::path::Path;

use anyhow::Result;
use rusqlite::params;

use crate::collection::folders::FolderId;
use crate::collection::music::database_api::MusicDbApi;
use crate::collection::music::{MusicItemDescription, MusicItemId};
use crate::database::sqlite::utils::{DatabaseUtils, ProtobufExporter, ProtobufImporter};

pub struct MusicDb {
    db_utils: DatabaseUtils,
}

impl MusicDb {
    pub fn new(db_utils: DatabaseUtils) -> Self {
        Self {
            db_utils,
        }
    }

    pub fn import(&self, base_path: &Path) -> Result<()> {
        let db_context = self.db_utils.lock();

        let mut importer = ProtobufImporter::create(base_path, "music_items.pb")?;
        while let Some(row) = importer.read_next_row::<crate::proto::collection::MusicItemsRow>()? {
            db_context.connection().execute(
                "INSERT INTO music_items (id, name, folder_id) VALUES (?1, ?2, ?3)",
                params![row.music_item_id, row.name, row.folder_id],
            )?;
        }

        Ok(())
    }

    pub fn export(&self, base_path: &Path) -> Result<()> {
        let db_context = self.db_utils.lock();

        let mut exporter = ProtobufExporter::create(base_path, "music_items.pb")?;
        let mut stmt = db_context.connection().prepare("SELECT id, name, folder_id FROM music_items")?;
        let rows = stmt.query_map([], |row| {
            let mut music_items_row = crate::proto::collection::MusicItemsRow::new();
            music_items_row.music_item_id = row.get::<_, i64>(0)?;
            music_items_row.name = row.get::<_, String>(1)?;
            music_items_row.folder_id = row.get::<_, i64>(2)?;
            Ok(music_items_row)
        })?;
        for row in rows {
            exporter.write_row(&row?)?;
        }

        Ok(())
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

    fn set_item_name(&self, item_id: MusicItemId, name: &str) -> Result<()> {
        let mut context = self.db_utils.lock();
        context.set_field_value(item_id, "music_items", "name", name)?;
        context.on_folders_updated(); // Notify any observers of the change
        Ok(())
    }

    fn get_music_item_description(&self, music_id: MusicItemId) -> Result<MusicItemDescription> {
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

    fn get_all_music_items(&self) -> Result<Vec<MusicItemId>> {
        self.db_utils.lock().get_rows_list("music_items")
    }

    fn get_music_item_folder(&self, item_id: MusicItemId) -> Result<FolderId> {
        self.db_utils.lock().get_field_value(item_id, "music_items","folder_id")
    }
}
