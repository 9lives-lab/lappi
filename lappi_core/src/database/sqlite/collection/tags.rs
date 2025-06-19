use std::borrow::BorrowMut;
use std::path::Path;

use anyhow::Result;
use rusqlite::{params, OptionalExtension};

use crate::database::sqlite::utils::{DatabaseContext, DatabaseUtils, ProtobufExporter, ProtobufImporter};
use crate::collection::folders::FolderId;
use crate::collection::music::MusicItemId;
use crate::collection::tags::{Tag, TagValue};
use crate::collection::tags::database_api::TagsDbApi;

struct TagsUtils<'a> {
    context: &'a mut DatabaseContext,
    id_field_name: &'static str,
}

impl <'a> TagsUtils<'a> {
    fn new_music_item_utils(context: &'a mut DatabaseContext) -> Self {
        Self {
            context,
            id_field_name: "music_item_id",
        }
    }

    fn new_folder_utils(context: &'a mut DatabaseContext) -> Self {
        Self {
            context,
            id_field_name: "folder_id",
        }
    }

    pub fn add_tag_row(&self, id_field_value: i64, tag_name: &str, tag_value: &TagValue) -> Result<()> {
        let sql = format!("INSERT INTO tags ({}, tag_name, string_value, int_value) VALUES (?1, ?2, ?3, ?4)", self.id_field_name);
        
        let (string_value, int_value): (Option<String>, Option<i32>) = match tag_value {
            TagValue::String(value) => (Some(value.clone()), None),
            TagValue::Number(value) => (None, Some(*value)),
            TagValue::Bool => (None, None),
        };
        
        self.context.connection().execute(
            sql.as_str(),
            params![id_field_value, tag_name, string_value, int_value],
        )?;
        Ok(())
    }

    pub fn get_tag_row_id(&self, id_field: i64, tag_name: &str) -> Result<Option<i64>> {
        let sql = format!("SELECT id FROM tags WHERE {}=(?1) AND tag_name=(?2)", self.id_field_name);
        let result = self.context.connection().query_row(&sql, params![id_field, tag_name],
            |row| row.get::<_, i64>(0),
        ).optional()?;
        Ok(result)
    }

    fn set_add_tag(&mut self, id_field_value: i64, tag_name: &str, tag_value: &TagValue) -> Result<()> {
        match self.get_tag_row_id(id_field_value, tag_name)? {
            Some(id) => {
                let sql = format!("UPDATE tags SET string_value=(?1), int_value=(?2) WHERE id=(?3)");
                let (string_value, int_value): (Option<String>, Option<i32>) = match tag_value {
                    TagValue::String(value) => (Some(value.clone()), None),
                    TagValue::Number(value) => (None, Some(*value)),
                    TagValue::Bool => (None, None),
                };

                self.context.connection().execute(
                    sql.as_str(),
                    params![string_value, int_value, id],
                )?;
            }
            None => {
                self.add_tag_row(id_field_value, tag_name, tag_value)?;
            }
        }
        self.context.on_music_updated();
        return Ok(());
    }

    fn get_tags(&self, id_field: i64) -> Result<Vec<Tag>> {
        let sql = format!("SELECT tag_name, string_value, int_value FROM tags WHERE {}=(?1)", self.id_field_name);
        let mut tags_stmt = self.context.connection().prepare(sql.as_str())?;
        let tags_rows = tags_stmt.query_map(params![id_field],|row| {
            let tag_name = row.get(0)?;
            let string_value: Option<String> = row.get(1)?;
            let int_value: Option<i32> = row.get(2)?;
            let tag_value = match (string_value, int_value) {
                (Some(value), None) => TagValue::String(value),
                (None, Some(value)) => TagValue::Number(value),
                (None, None) => TagValue::Bool,
                _ => return Err(rusqlite::Error::InvalidQuery),
            };
            Ok(Tag::new(tag_name, tag_value))
        })?;
        Ok(tags_rows.collect::<Result<Vec<_>, _>>()?)
    }

    fn get_tag(&self, id_field: i64, tag_name: &str) -> Result<Option<Tag>> {
        let tags = self.get_tags(id_field)?;
        let tag = tags.iter().find(|&tag| tag.get_key().eq(tag_name));
        return Ok(tag.map(|x| x.clone()));
    }

    fn delete_tag(&mut self, id_field: i64, tag_name: &str) -> Result<()> {
        let sql = format!("DELETE FROM tags WHERE {}=(?1) AND tag_name=(?2)", self.id_field_name);
        self.context.connection().execute(
            sql.as_str(),
            params![id_field, tag_name],
        )?;
        self.context.on_music_updated();
        return Ok(());
    }
}

pub struct TagsDb {
    db_utils: DatabaseUtils,
}

impl TagsDb {
    pub fn new(db_utils: DatabaseUtils) -> Self {
        Self {
            db_utils,
        }
    }

    pub fn import(&self, base_path: &Path) -> Result<()> {
        let db_context = self.db_utils.lock();

        let mut importer = ProtobufImporter::create(base_path, "tags.pb")?;
        while let Some(row) = importer.read_next_row::<crate::proto::collection::TagsRow>()? {
            db_context.connection().execute(
                "INSERT INTO tags (id, music_item_id, folder_id, tag_name, string_value, int_value) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![row.tag_id, row.music_item_id, row.folder_id, row.tag_name, row.string_value, row.int_value],
            )?;
        }

        Ok(())
    }

    pub fn export(&self, base_path: &Path) -> Result<()> {
        let db_context = self.db_utils.lock();
        let mut exporter = ProtobufExporter::create(base_path, "tags.pb")?;
        let mut stmt = db_context.connection().prepare("SELECT id, music_item_id, folder_id, tag_name, string_value, int_value FROM tags")?;
        let rows = stmt.query_map([], |row| {
            let mut tag_row = crate::proto::collection::TagsRow::new();
            tag_row.tag_id = row.get::<_, i64>(0)?;
            tag_row.music_item_id = row.get::<_, Option<i64>>(1)?;
            tag_row.folder_id = row.get::<_, Option<i64>>(2)?;
            tag_row.tag_name = row.get::<_, String>(3)?;
            tag_row.string_value = row.get::<_, Option<String>>(4)?;
            tag_row.int_value = row.get::<_, Option<i32>>(5)?;
            Ok(tag_row)
        })?;
        for row in rows {
            exporter.write_row(&row?)?;
        }
        Ok(())
    }
}

impl TagsDbApi for TagsDb {
    fn clone_api(&self) -> Box<dyn TagsDbApi> {
        return Box::new(TagsDb::new(self.db_utils.clone()));
    }

    fn set_add_item_tag(&self, item_id: MusicItemId, tag_name: &str, tag_value: &TagValue) -> Result<()> {
        let mut context = self.db_utils.lock();
        let mut tags_utils = TagsUtils::new_music_item_utils(context.borrow_mut());
        return tags_utils.set_add_tag(item_id, tag_name, tag_value);
    }

    fn get_item_tag(&self, item_id: MusicItemId, tag_name: &str) -> Result<Option<Tag>> {
        let mut context = self.db_utils.lock();
        let tags_utils = TagsUtils::new_music_item_utils(context.borrow_mut());
        return tags_utils.get_tag(item_id, tag_name);
    }

    fn get_item_tags(&self, item_id: MusicItemId) -> Result<Vec<Tag>> {
        let mut context = self.db_utils.lock();
        let tags_utils = TagsUtils::new_music_item_utils(context.borrow_mut());
        return tags_utils.get_tags(item_id);
    }

    fn delete_item_tag(&self, item_id: MusicItemId, tag_name: &str) -> Result<()> {
        let mut context = self.db_utils.lock();
        let mut tags_utils = TagsUtils::new_music_item_utils(context.borrow_mut());
        return tags_utils.delete_tag(item_id, tag_name);
    }

    fn set_add_folder_tag(&self, folder_id: FolderId, tag_name: &str, tag_value: &TagValue) -> Result<()> {
        let mut context = self.db_utils.lock();
        let mut tags_utils = TagsUtils::new_folder_utils(context.borrow_mut());
        return tags_utils.set_add_tag(folder_id, tag_name, tag_value);
    }

    fn get_folder_tag(&self, folder_id: FolderId, tag_name: &str) -> Result<Option<Tag>> {
        let mut context = self.db_utils.lock();
        let tags_utils = TagsUtils::new_folder_utils(context.borrow_mut());
        return tags_utils.get_tag(folder_id, tag_name);
    }

    fn get_folder_tags(&self, folder_id: FolderId) -> Result<Vec<Tag>> {
        let mut context = self.db_utils.lock();
        let tags_utils = TagsUtils::new_folder_utils(context.borrow_mut());
        return tags_utils.get_tags(folder_id);
    }

    fn delete_folder_tag(&self, folder_id: FolderId, tag_name: &str) -> Result<()> {
        let mut context = self.db_utils.lock();
        let mut tags_utils = TagsUtils::new_folder_utils(context.borrow_mut());
        return tags_utils.delete_tag(folder_id, tag_name);
    }
}
