use std::sync::{Mutex, Arc};

use rusqlite::{params, Connection};
use amina_core::service::Context;

use crate::database_api::{DbExporter, DbImporter, DbResult};
use crate::collection::tree::{FolderDescription, FolderType};
use crate::collection::database_api::DatabaseApi;
use crate::collection::music::types::ExternalSrcFileDesc;
use crate::collection::types::tags::Tag;
use crate::collection::types::{ArtistId, EdgeId, FolderId, ItemId, ItemType};
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

    fn add_collection_item(&self) -> ItemId {
        let context = self.db_utils.lock();
        let creation_date = 0i32;
        context.connection().execute(
            "INSERT INTO collection_items (creation_date) VALUES (?1)",
            params![creation_date],
        ).unwrap();
        return context.connection().last_insert_rowid();
    }

    fn get_collection_items(&self) -> DbResult<Vec<ItemId>> {
        self.db_utils.lock().get_rows_list("collection_items")
    }

    fn find_or_add_artist(&self, name: &str) -> DbResult<ArtistId> {
        self.db_utils.lock().find_or_add_string_row("artist_entries", "name", name)
    }

    fn get_artist_name(&self, artist_id: ArtistId) -> DbResult<String> {
        let context = self.db_utils.lock();
        utils::artists::get_artist_name(&context, artist_id)
    }

    fn add_artist_to_collection_item(&self, artist_id: ArtistId, item_id: ItemId) -> DbResult<()> {
        let context = self.db_utils.lock();
        utils::artists::add_artist_to_collection_item(&context, artist_id, item_id)
    }

    fn get_artists_by_collection_item(&self, item_id: ItemId) -> DbResult<Vec<ArtistId>> {
        let context = self.db_utils.lock();
        utils::artists::get_artists_by_collection_item(&context, item_id)
    }

    fn add_node(&self, item_type: ItemType, name: &str) -> ItemId {
        let context = self.db_utils.lock();
        return utils::nodes::add_node(context.connection(), item_type, name);
    }

    fn find_or_add_node(&self, item_type: ItemType, name: &str) -> ItemId {
        let context = self.db_utils.lock();
        match utils::nodes::find_node_id(context.connection(), item_type.clone(), name) {
            Some(id) => id,
            None => {
                utils::nodes::add_node(context.connection(), item_type, name)
            }
        }
    }

    fn get_node_type(&self, item_id: ItemId) -> ItemType {
        let context = self.db_utils.lock();
        return utils::nodes::get_node_type(context.connection(), item_id);
    }

    fn get_node_name(&self, item_id: ItemId) -> String {
        let context = self.db_utils.lock();
        return utils::nodes::get_node_name(context.connection(), item_id);
    }

    fn get_all_nodes(&self) -> Vec<ItemId> {
        let context = self.db_utils.lock();
        let mut collection_stmt = context.connection().prepare("SELECT id FROM collection_nodes").unwrap();
        let collection_rows = collection_stmt.query_map(
            [],|row| row.get::<_, i64>(0)
        ).unwrap();
        let mut result = Vec::new();
        for collection_item in collection_rows {
            result.push(collection_item.unwrap());
        }

        return result;
    }

    fn find_or_add_edge(&self, first_node: ItemId, second_node: ItemId) -> EdgeId {
        let context = self.db_utils.lock();
        match utils::nodes::find_edge_id(context.connection(), first_node, second_node) {
            Some(id) => id,
            None => {
                utils::nodes::add_edge(context.connection(), first_node, second_node)
            }
        }
    }

    fn get_edge(&self, edge_id: EdgeId) -> (ItemId, ItemId) {
        let context = self.db_utils.lock();
        return utils::nodes::get_edge(context.connection(), edge_id);
    }

    fn get_all_edges(&self) -> Vec<EdgeId> {
        let context = self.db_utils.lock();
        return utils::nodes::get_all_edges(context.connection());
    }

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
                  WHERE tags_values.id IN (SELECT tag_id FROM tags WHERE item_id=(?1))"
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

    fn get_root_folder(&self) -> FolderId {
        utils::get_root_folder()
    }

    fn get_item_folder(&self, item_id: ItemId) -> DbResult<FolderId> {
        self.db_utils.lock().get_field_value(item_id, "collection_items","folder_id")
    }

    fn get_folder_parent(&self, folder_id: FolderId) -> DbResult<FolderId> {
        let context = self.db_utils.lock();
        let result = context.connection().query_row(
            "SELECT parent_id FROM collection_folders WHERE id=(?1)",
            params![folder_id],
            |row| row.get::<_, i64>(0),
        )?;
        Ok(result)
    }

    fn get_folder_description(&self, folder_id: FolderId) -> DbResult<FolderDescription> {
        let context = self.db_utils.lock();
        utils::tree::get_folder_description(&context, folder_id)
    }

    fn find_or_add_folder(&self, parent_id: FolderId, folder_name: &str, folder_type: FolderType) -> DbResult<FolderId> {
        let context = self.db_utils.lock();
        utils::tree::find_add_folder_id(context.connection(), parent_id, folder_name, folder_type)
    }

    fn get_folders_in_folder(&self, folder_id: FolderId) -> DbResult<Vec<FolderDescription>> {
        let context = self.db_utils.lock();
        utils::tree::get_folders_in_folder(&context, folder_id)
    }

    fn set_folder_for_artist(&self, artist_id: ArtistId, folder_id: FolderId) -> DbResult<()> {
        let context = self.db_utils.lock();
        utils::tree::set_folder_for_artist(context.connection(), artist_id, folder_id)
    }

    fn set_folder_for_item(&self, item_id: ItemId, folder_id: FolderId) -> DbResult<()> {
        let context = self.db_utils.lock();
        utils::tree::set_folder_for_item(context.connection(), item_id, folder_id)
    }

    fn get_items_in_folder(&self, folder_id: FolderId) -> DbResult<Vec<ItemId>> {
        let context = self.db_utils.lock();
        utils::tree::get_items_in_folder(context.connection(), folder_id)
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
