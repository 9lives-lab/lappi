use std::borrow::BorrowMut;

use rusqlite::{params, OptionalExtension};

use crate::database::sqlite::utils::{DatabaseContext, DatabaseUtils};
use crate::database::api::DbResult;
use crate::collection::folders::FolderId;
use crate::collection::music::MusicItemId;
use crate::collection::tags::Tag;
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

    pub fn add_tag_row(&self, id_field_value: i64, tag_name: &str, tag_value: &str) -> DbResult<()> {
        let sql = format!("INSERT INTO tags ({}, tag_name, tag_value) VALUES (?1, ?2, ?3)", self.id_field_name);
        self.context.connection().execute(
            sql.as_str(),
            params![id_field_value, tag_name, tag_value],
        )?;
        Ok(())
    }

    pub fn get_tag_row_id(&self, id_field: i64, tag_name: &str) -> DbResult<Option<i64>> {
        let sql = format!("SELECT id FROM tags WHERE {}=(?1) AND tag_name=(?2)", self.id_field_name);
        let result = self.context.connection().query_row(&sql, params![id_field, tag_name],
            |row| row.get::<_, i64>(0),
        ).optional()?;
        Ok(result)
    }

    fn set_add_tag(&mut self, id_field_value: i64, tag_name: &str, tag_value: &str) -> DbResult<()> {
        match self.get_tag_row_id(id_field_value, tag_name)? {
            Some(id) => {
                let sql = format!("UPDATE tags SET tag_value=(?1) WHERE id=(?2)");
                self.context.connection().execute(
                    sql.as_str(),
                    params![tag_value, id],
                )?;
            }
            None => {
                self.add_tag_row(id_field_value, tag_name, tag_value)?;
            }
        }
        self.context.on_music_updated();
        return Ok(());
    }

    fn get_tags(&self, id_field: i64) -> DbResult<Vec<Tag>> {
        let sql = format!("SELECT tag_name, tag_value FROM tags WHERE {}=(?1)", self.id_field_name);
        let mut tags_stmt = self.context.connection().prepare(sql.as_str())?;
        let tags_rows = tags_stmt.query_map(params![id_field],|row| {
            let key = row.get::<_, String>(0)?;
            let value = row.get::<_, String>(1)?;
            Ok(Tag::new_string(key, value))
        })?;
        Ok(tags_rows.map(|x| x.unwrap()).collect())
    }

    fn get_tag(&self, id_field: i64, tag_name: &str) -> DbResult<Option<Tag>> {
        let tags = self.get_tags(id_field)?;
        let tag = tags.iter().find(|&tag| tag.get_key().eq(tag_name));
        return Ok(tag.map(|x| x.clone()));
    }

    fn delete_tag(&mut self, id_field: i64, tag_name: &str) -> DbResult<()> {
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
}

impl TagsDbApi for TagsDb {
    fn clone_api(&self) -> Box<dyn TagsDbApi> {
        return Box::new(TagsDb::new(self.db_utils.clone()));
    }

    fn set_add_item_tag(&self, item_id: MusicItemId, tag_name: &str, tag_value: &str) -> DbResult<()> {
        let mut context = self.db_utils.lock();
        let mut tags_utils = TagsUtils::new_music_item_utils(context.borrow_mut());
        return tags_utils.set_add_tag(item_id, tag_name, tag_value);
    }

    fn get_item_tag(&self, item_id: MusicItemId, tag_name: &str) -> DbResult<Option<Tag>> {
        let mut context = self.db_utils.lock();
        let tags_utils = TagsUtils::new_music_item_utils(context.borrow_mut());
        return tags_utils.get_tag(item_id, tag_name);
    }

    fn get_item_tags(&self, item_id: MusicItemId) -> DbResult<Vec<Tag>> {
        let mut context = self.db_utils.lock();
        let tags_utils = TagsUtils::new_music_item_utils(context.borrow_mut());
        return tags_utils.get_tags(item_id);
    }

    fn delete_item_tag(&self, item_id: MusicItemId, tag_name: &str) -> DbResult<()> {
        let mut context = self.db_utils.lock();
        let mut tags_utils = TagsUtils::new_music_item_utils(context.borrow_mut());
        return tags_utils.delete_tag(item_id, tag_name);
    }

    fn set_add_folder_tag(&self, folder_id: FolderId, tag_name: &str, tag_value: &str) -> DbResult<()> {
        let mut context = self.db_utils.lock();
        let mut tags_utils = TagsUtils::new_folder_utils(context.borrow_mut());
        return tags_utils.set_add_tag(folder_id, tag_name, tag_value);
    }

    fn get_folder_tag(&self, folder_id: FolderId, tag_name: &str) -> DbResult<Option<Tag>> {
        let mut context = self.db_utils.lock();
        let tags_utils = TagsUtils::new_folder_utils(context.borrow_mut());
        return tags_utils.get_tag(folder_id, tag_name);
    }

    fn get_folder_tags(&self, folder_id: FolderId) -> DbResult<Vec<Tag>> {
        let mut context = self.db_utils.lock();
        let tags_utils = TagsUtils::new_folder_utils(context.borrow_mut());
        return tags_utils.get_tags(folder_id);
    }

    fn delete_folder_tag(&self, folder_id: FolderId, tag_name: &str) -> DbResult<()> {
        let mut context = self.db_utils.lock();
        let mut tags_utils = TagsUtils::new_folder_utils(context.borrow_mut());
        return tags_utils.delete_tag(folder_id, tag_name);
    }
}
