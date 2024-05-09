use rusqlite::{params, OptionalExtension};

use crate::database_api::DbResult;
use crate::collection::types::{FolderId};
use crate::collection::folders::{FolderDescription, FolderType};
use crate::database::sqlite::utils;
use crate::database::sqlite::utils::DatabaseContext;

fn i32_to_folder_type(i: i32) -> FolderType {
    match i {
        0 => FolderType::Folder,
        1 => FolderType::Artist,
        2 => FolderType::Album,
        _ => panic!("Unexpected folder type {}", i),
    }
}

pub fn get_folder_description(context: &DatabaseContext, folder_id: FolderId) -> DbResult<FolderDescription> {
    if folder_id == utils::get_root_folder() {
        return Ok(FolderDescription {
            folder_id: 0,
            name: String::from("Root"),
            folder_type: FolderType::Folder,
            avatar_picture_id: None,
        });
    }

    let folder_description = context.connection().query_row(
        "SELECT id, name, folder_type, avatar_picture_id FROM folders WHERE id=(?1)",
        params![folder_id],
        |row| {
            Ok(FolderDescription {
                folder_id:   row.get:: < _, i64>(0)? as FolderId,
                name:        row.get:: < _, String>(1)?,
                folder_type: i32_to_folder_type(row.get:: < _, i32>(2)?),
                avatar_picture_id: row.get:: < _, Option<i64>>(3)?,
            })
        },
    )?;

    Ok(folder_description)
}

pub fn find_folder_id(context: &DatabaseContext, parent_id: FolderId, folder_name: &str) -> DbResult<Option<FolderId>> {
    let result = context.connection().query_row(
        "SELECT id FROM folders WHERE parent_id == (?1) AND name == (?2)",
        params![parent_id, folder_name],
        |row| row.get::<_, i64>(0),
    ).optional()?;
    Ok(result)
}

pub fn find_add_folder_id(context: &mut DatabaseContext, parent_id: FolderId, folder_name: &str, folder_type: FolderType) -> DbResult<i64> {
    let folder_id = match find_folder_id(context, parent_id, folder_name)? {
        Some(id) => id,
        None => {
            context.connection().execute(
                "INSERT INTO folders (parent_id, name, folder_type) VALUES (?1, ?2, ?3)",
                params![parent_id, folder_name, folder_type as i32],
            )?;
            context.on_collection_updated();
            context.connection().last_insert_rowid()
        }
    };
    Ok(folder_id)
}

pub fn get_folders_in_folder(context: &DatabaseContext, folder_id: FolderId) -> DbResult<Vec<FolderDescription>> {
    let id_list = context.get_fields_list_by_field_i64_value("folders", "id", "parent_id", folder_id).unwrap();
    let mut result = Vec::new();
    for folder_id in id_list {
        result.push(get_folder_description(context, folder_id)?);
    }
    Ok(result)
}

