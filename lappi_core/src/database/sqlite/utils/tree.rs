use rusqlite::{params, Connection, OptionalExtension};

use crate::database_api::DbResult;
use crate::collection::types::{ArtistId, FolderId, ItemId};
use crate::collection::tree::{FolderDescription, FolderDetails, FolderType};
use crate::database::sqlite::utils;
use crate::database::sqlite::utils::DatabaseContext;

fn i32_to_folder_type(i: i32) -> FolderType {
    match i {
        0 => FolderType::Tag,
        1 => FolderType::Artist,
        _ => panic!("Unexpected folder type {}", i),
    }
}

#[allow(dead_code)]
pub fn get_all_music_collection_items(conn: &Connection) -> DbResult<Vec<i64>> {
    let mut collection_stmt = conn.prepare("SELECT id FROM collection_items")?;
    let collection_rows = collection_stmt.query_map(
        [],|row| row.get::<_, i64>(0)
    )?;
    let mut result = Vec::new();
    for collection_item in collection_rows {
        result.push(collection_item.unwrap());
    }
    return Ok(result);
}

pub fn get_folder_description(context: &DatabaseContext, folder_id: FolderId) -> DbResult<FolderDescription> {
    if folder_id == utils::get_root_folder() {
        return Ok(FolderDescription {
            folder_id: 0,
            title: String::from("Root"),
            folder_type: FolderType::Tag,
            details: FolderDetails::None,
        });
    }

    let mut folder_description = context.connection().query_row(
        "SELECT id, name, folder_type FROM collection_folders WHERE id=(?1)",
        params![folder_id],
        |row| {
            Ok(FolderDescription {
                folder_id:   row.get:: < _, i64>(0)? as FolderId,
                title:       row.get:: < _, String>(1)?,
                folder_type: i32_to_folder_type(row.get:: < _, i32>(2)?),
                details:     FolderDetails::None,
            })
        },
    )?;

    if folder_description.folder_type == FolderType::Artist {
        let artist_id = context.get_field_by_key("artist_entries", "folder_id", folder_id, "id")?;
        folder_description.details = FolderDetails::Artist(artist_id);
    }

    Ok(folder_description)
}

pub fn find_folder_id(conn: &Connection, parent_id: FolderId, folder_name: &str) -> DbResult<Option<FolderId>> {
    let result = conn.query_row(
        "SELECT id FROM collection_folders WHERE parent_id == (?1) AND name == (?2)",
        params![parent_id, folder_name],
        |row| row.get::<_, i64>(0),
    ).optional()?;
    Ok(result)
}

pub fn find_add_folder_id(conn: &Connection, parent_id: FolderId, folder_name: &str, folder_type: FolderType) -> DbResult<i64> {
    let folder_id = match find_folder_id(conn, parent_id, folder_name)? {
        Some(id) => id,
        None => {
            conn.execute(
                "INSERT INTO collection_folders (parent_id, name, folder_type) VALUES (?1, ?2, ?3)",
                params![parent_id, folder_name, folder_type as i32],
            )?;
            conn.last_insert_rowid()
        }
    };
    Ok(folder_id)
}

pub fn get_folders_in_folder(context: &DatabaseContext, folder_id: FolderId) -> DbResult<Vec<FolderDescription>> {
    let id_list = context.get_fields_list_by_field_i64_value("collection_folders", "id", "parent_id", folder_id).unwrap();
    let mut result = Vec::new();
    for folder_id in id_list {
        result.push(get_folder_description(context, folder_id)?);
    }
    Ok(result)
}

pub fn get_items_in_folder(conn: &Connection, folder_id: FolderId) -> DbResult<Vec<ItemId>> {
    let mut items_stmt = conn.prepare(
        "SELECT id FROM collection_items WHERE folder_id=(?1)"
    )?;
    let items_rows = items_stmt.query_map(
        params![folder_id],
        |row| row.get::< _, i64>(0).map(|p| p as ItemId)
    )?;
    Ok(items_rows.map(|p| p.unwrap()).collect())
}

pub fn set_folder_for_artist(conn: &Connection, artist_id: ArtistId, folder_id: FolderId) -> DbResult<()> {
    conn.execute(
        "UPDATE artist_entries SET folder_id=(?1) WHERE id=(?2)",
        params![folder_id, artist_id],
    )?;
    Ok(())
}

pub fn set_folder_for_item(conn: &Connection, item_id: ItemId, folder_id: FolderId) -> DbResult<()> {
    conn.execute(
        "UPDATE collection_items SET folder_id=(?1) WHERE id=(?2)",
        params![folder_id, item_id],
    )?;
    Ok(())
}
