use crate::collection::types::{ArtistId, ItemId};
use crate::database::sqlite::utils::DatabaseContext;
use crate::database_api::DbResult;

pub fn find_or_add_artist(context: &DatabaseContext, name: &str) -> DbResult<ArtistId> {
    context.find_or_add_string_row("artist_entries", "name", name)
}

pub fn get_artist_name(context: &DatabaseContext, artist_id: ArtistId) -> DbResult<String> {
    context.get_field_value(artist_id, "artist_entries",  "name")
}

pub fn add_artist_to_collection_item(context: &DatabaseContext, artist_id: ArtistId, item_id: ItemId) -> DbResult<()> {
    context.add_connection("artist_connections", "artist_id", artist_id, "item_id", item_id)
}

pub fn get_artists_by_collection_item(context: &DatabaseContext, item_id: ItemId) -> DbResult<Vec<ArtistId>> {
    context.get_fields_list_by_field_i64_value("artist_connections", "artist_id", "item_id", item_id)
}
