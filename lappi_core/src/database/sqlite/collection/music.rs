use num_traits::FromPrimitive;
use rusqlite::{params, Connection, OptionalExtension};

use crate::collection::folders::FolderId;
use crate::collection::music::database_api::MusicDbApi;
use crate::collection::music::{MusicItemDescription, MusicItemId, MusicSourceFileId, SourceFileDesc, SourceType, Tag};
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

    pub fn add_tag_to_item(&self, conn: &Connection, collection_item_id: i64, tag_value_id: i64) -> DbResult<()> {
        conn.execute(
            "INSERT INTO music_items_tags (item_id, tag_id) VALUES (?1, ?2)",
            params![collection_item_id, tag_value_id],
        )?;
        Ok(())
    }
    
    pub fn get_add_tag_value(&self, conn: &Connection, name_id: i64, value: &str) -> DbResult<i64> {
        let result = conn.query_row(
            "SELECT id FROM tags_values WHERE name_id=(?1) AND value=(?2)",
            params![name_id, value],
            |row| row.get::<_, i64>(0),
        ).optional()?;
        let value_id = match result {
            Some(id) => id,
            None => {
                conn.execute(
                    "INSERT INTO tags_values (name_id, value) VALUES (?1, ?2)",
                    params![name_id, value],
                )?;
                conn.last_insert_rowid()
            }
        };
        Ok(value_id)
    }
    
    pub fn get_tag_name_id(&self, conn: &Connection, name: &str) -> DbResult<Option<i64>> {
        let result = conn.query_row(
            "SELECT id FROM tags_names WHERE name=(?1)",
            params![name],
            |row| row.get::<_, i64>(0),
        ).optional()?;
        Ok(result)
    }
    
    pub fn get_add_tag_name(&self, conn: &Connection, name: &str) -> DbResult<i64> {
        let name_id = match self.get_tag_name_id(conn, name)? {
            Some(id) => id,
            None => {
                conn.execute(
                    "INSERT INTO tags_names (name) VALUES (?1)",
                    params![name],
                )?;
                conn.last_insert_rowid()
            }
        };
        Ok(name_id)
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

    fn add_tag(&self, item_id: MusicItemId, name: &str, value: &str) -> DbResult<()> {
        let mut context = self.db_utils.lock();
        let conn = context.connection();
        let tag_name_id = self.get_add_tag_name(&conn, name)?;
        let tag_value_id = self.get_add_tag_value(&conn, tag_name_id, value)?;
        let result = self.add_tag_to_item(&conn, item_id, tag_value_id);
        
        context.on_music_updated();
        return result;
    }

    fn get_tag(&self, item_id: MusicItemId, key: &str) -> DbResult<Option<Tag>> {
        let tags = self.get_tags(item_id)?;
        let tag = tags.iter().find(|&tag| tag.get_key().eq(key));
        return Ok(tag.map(|x| x.clone()));
    }

    fn get_tags(&self, item_id: MusicItemId) -> DbResult<Vec<Tag>> {
        let context = self.db_utils.lock();
        let mut tags_stmt = context.connection().prepare(
            "SELECT tags_names.name, tags_values.value
                  FROM tags_values
                  INNER JOIN tags_names ON tags_names.id = tags_values.name_id
                  WHERE tags_values.id IN (SELECT tag_id FROM music_items_tags WHERE item_id=(?1))"
        )?;
        let tags_rows = tags_stmt.query_map(
            params![item_id],|row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?
                ))
            }
        )?;
        let mut tags_vec = Vec::new();
        for tag in tags_rows {
            let tag = tag?;
            tags_vec.push(Tag::new_string(tag.0, tag.1));
        }
        return Ok(tags_vec);
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
            "SELECT id, path, source_type FROM music_src_files WHERE music_item_id=(?1)"
        )?;
        let rows = stmt.query_map(
            params![item_id], |row| Ok(
                SourceFileDesc {
                    id: row.get::<_, i32>(0)? as MusicItemId,
                    path: row.get::<_, String>(1)?,
                    source_type: SourceType::from_i32(row.get::<_, i32>(2)?).unwrap(),
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
