use rusqlite::params;
use crate::collection::types::{PictureId, FolderId};
use crate::database::sqlite::utils::DatabaseContext;
use crate::database_api::DbResult;

pub fn add_picture(context: &DatabaseContext, extension: &str, folder_id: FolderId) -> DbResult<PictureId> {
    let query = "INSERT INTO picture_items (extension, folder_id) VALUES (?1, ?2)";
    context.connection.execute(&query, params![extension, folder_id])?;
    Ok(context.connection.last_insert_rowid())
}

pub fn get_picture_extension(context: &DatabaseContext, picture_id: PictureId) -> DbResult<String> {
    context.get_field_value(picture_id, "picture_items", "extension")
}
