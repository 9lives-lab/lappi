use crate::collection::types::{PictureId, ArtistId};
use crate::database::sqlite::utils::DatabaseContext;
use crate::database_api::DbResult;

pub fn add_picture(context: &DatabaseContext, extension: &str) -> DbResult<PictureId> {
    let query = "INSERT INTO pictures_entries (extension) VALUES (?1)";
    context.connection.execute(&query, [extension])?;
    Ok(context.connection.last_insert_rowid())
}

pub fn add_picture_to_artist(context: &DatabaseContext, picture_id: PictureId, artist_id: ArtistId) -> DbResult<()> {
    context.add_connection("artist_pictures", "artist_id", artist_id, "picture_id", picture_id)
}

pub fn get_pictures_by_artist(context: &DatabaseContext, artist_id: ArtistId) -> DbResult<Vec<PictureId>> {
    context.get_fields_list_by_field_i64_value("artist_pictures", "picture_id", "artist_id", artist_id)
}

pub fn get_picture_extension(context: &DatabaseContext, picture_id: PictureId) -> DbResult<String> {
    context.get_field_value(picture_id, "pictures_entries", "extension")
}
