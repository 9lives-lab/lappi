use std::sync::{Mutex, Arc};

use rusqlite::{params, Connection};
use amina_core::service::Context;

use crate::database_api::{DbExporter, DbImporter, DbResult};
use crate::collection::folders::{FolderDescription, FolderType};
use crate::collection::database_api::DatabaseApi;
use crate::collection::music::types::ExternalSrcFileDesc;
use crate::collection::types::tags::Tag;
use crate::collection::types::{FolderId, ItemId, MusicItemId, PictureId};
use crate::database::sqlite::utils::DatabaseUtils;
use super::utils;

pub struct CollectionDbApi {
    db_utils: DatabaseUtils,
}

impl CollectionDbApi {

    pub fn new(context: &Context, connection: Arc<Mutex<Connection>>) -> Self {
        Self {
            db_utils: DatabaseUtils::new(context, connection),
        }
    }

}

impl DatabaseApi for CollectionDbApi {

    fn clone_api(&self) -> Box<dyn DatabaseApi> {
        Box::new(CollectionDbApi {
            db_utils: self.db_utils.clone(),
        })
    }

    fn start_batch(&self) {
        self.db_utils.lock().start_batch();
    }

    fn stop_batch(&self) {
        self.db_utils.lock().stop_batch();
    }

    // Folders

    fn get_root_folder(&self) -> FolderId {
        utils::get_root_folder()
    }

    fn get_folder_parent(&self, folder_id: FolderId) -> DbResult<FolderId> {
        self.db_utils.lock().get_field_value(folder_id, "folders", "parent_id")
    }

    fn get_folder_name(&self, folder_id: FolderId) -> DbResult<String> {
        self.db_utils.lock().get_field_value(folder_id, "folders", "name")
    }

    fn get_folder_description(&self, folder_id: FolderId) -> DbResult<FolderDescription> {
        let context = self.db_utils.lock();
        utils::tree::get_folder_description(&context, folder_id)
    }

    fn find_or_add_folder(&self, parent_id: FolderId, folder_name: &str, folder_type: FolderType) -> DbResult<FolderId> {
        let mut context = self.db_utils.lock();
        let folder_id = utils::tree::find_add_folder_id(&mut context, parent_id, folder_name, folder_type)?;
        context.on_collection_updated();
        return Ok(folder_id);
    }

    fn get_folders_in_folder(&self, folder_id: FolderId) -> DbResult<Vec<FolderDescription>> {
        let context = self.db_utils.lock();
        utils::tree::get_folders_in_folder(&context, folder_id)
    }

    // Music items

    fn add_music_item(&self, name: &str, folder_id: FolderId) -> ItemId {
        let context = self.db_utils.lock();
        context.connection().execute(
            "INSERT INTO music_items (name, folder_id) VALUES (?1, ?2)",
            params![name, folder_id],
        ).unwrap();
        return context.connection().last_insert_rowid();
    }

    fn get_all_music_items(&self) -> DbResult<Vec<MusicItemId>> {
        self.db_utils.lock().get_rows_list("music_items")
    }

    fn get_music_item_folder(&self, item_id: MusicItemId) -> DbResult<FolderId> {
        self.db_utils.lock().get_field_value(item_id, "music_items","folder_id")
    }

    fn get_music_items_in_folder(&self, folder_id: FolderId) -> DbResult<Vec<MusicItemId>> {
        self.db_utils.lock().get_fields_list_by_field_i64_value("music_items", "id", "folder_id", folder_id)
    }

    // Pictures

    fn add_picture_item(&self, extension: &str, folder_id: FolderId) -> DbResult<PictureId> {
        let context = self.db_utils.lock();
        utils::pictures::add_picture(&context, extension, folder_id)
    }

    fn get_picture_extension(&self, picture_id: PictureId) -> DbResult<String> {
        let context = self.db_utils.lock();
        utils::pictures::get_picture_extension(&context, picture_id)
    }

    fn get_pictures_in_folder(&self, folder_id: FolderId) -> DbResult<Vec<PictureId>>{
        self.db_utils.lock().get_fields_list_by_field_i64_value("picture_items", "id", "folder_id", folder_id)
    }

    // Tags

    fn add_tag(&self, collection_item_id: ItemId, name: &str, value: &str) -> DbResult<()> {
        let mut context = self.db_utils.lock();
        let conn = context.connection();
        let tag_name_id = utils::tags::get_add_tag_name(&conn, name)?;
        let tag_value_id = utils::tags::get_add_tag_value(&conn, tag_name_id, value)?;
        let result = utils::tags::add_tag_to_item(&conn, collection_item_id, tag_value_id);
        
        context.on_tags_updated(collection_item_id);
        return result;
    }

    fn get_tag(&self, item_id: ItemId, key: &str) -> DbResult<Option<Tag>> {
        let tags = self.get_tags(item_id)?;
        let tag = tags.iter().find(|&tag| tag.get_key().eq(key));
        return Ok(tag.map(|x| x.clone()));
    }

    fn get_tags(&self, item_id: ItemId) -> DbResult<Vec<Tag>> {
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

    // Song files

    fn add_external_src_file(&self, item_id: ItemId, path: &str) -> DbResult<()> {
        let context = self.db_utils.lock();
        context.connection().execute(
            "INSERT INTO external_src_files (item_id, path) VALUES (?1, ?2)",
            params![item_id, path],
        )?;
        Ok(())
    }

    fn get_external_src_files(&self, item_id: ItemId) -> DbResult<Vec<ExternalSrcFileDesc>> {
        let context = self.db_utils.lock();
        let mut stmt = context.connection().prepare(
            "SELECT id, path FROM external_src_files WHERE item_id=(?1)"
        )?;
        let rows = stmt.query_map(
            params![item_id],|row| Ok(
                ExternalSrcFileDesc {
                    id:   row.get:: < _, i32>(0)? as ItemId,
                    path: row.get:: < _, String>(1)?
                }
            )
        )?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    fn export(&self, exporter: Box<dyn DbExporter>) -> DbResult<()> {
        let context = self.db_utils.lock();
        let tables_list = super::init::get_tables_list();
        for table_name in tables_list {
            context.export_table(table_name, exporter.as_ref())?;
        }
        Ok(())
    }

    fn import(&self, importer: Box<dyn DbImporter>) -> DbResult<()> {
        let context = self.db_utils.lock();
        let tables_list = super::init::get_tables_list();
        for table_name in tables_list {
            context.import_table(table_name, importer.as_ref())?;
            log::debug!("Imported table {}", table_name);
        }
        Ok(())
    }
}
