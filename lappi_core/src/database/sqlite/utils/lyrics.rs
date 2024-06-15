use rusqlite::{params};
use fallible_iterator::{FallibleIterator};
use crate::collection::lyrics::LyricsDescription;
use crate::collection::types::{LyricsId, MusicItemId};
use crate::database::sqlite::utils::DatabaseContext;
use crate::database_api::DbResult;

pub fn add_lyrics_item(context: &DatabaseContext, music_item_id: MusicItemId, lang_code: &str) -> DbResult<LyricsId> {
    let query = "INSERT INTO lyrics_items (music_item_id, lang_code) VALUES (?1, ?2)";
    context.connection.execute(&query, params![music_item_id, lang_code])?;
    Ok(context.connection.last_insert_rowid())
}

pub fn get_lyrics_list(context: &DatabaseContext, music_id: MusicItemId) -> DbResult<Vec<LyricsDescription>> {
    let mut stmt = context.connection().prepare("SELECT id, lang_code FROM lyrics_items WHERE music_item_id=(?1)")?;
    let rows = stmt.query([music_id])?;
    let rows = rows.map(|row| {
        Ok(LyricsDescription {
            lyrics_id: row.get::< _, i64>(0)? as LyricsId,
            lang_code: row.get::< _, String>(1)?,
        })
    });
    Ok(rows.collect()?)
}
