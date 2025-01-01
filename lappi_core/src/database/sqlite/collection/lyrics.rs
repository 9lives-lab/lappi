use anyhow::Result;
use rusqlite::params;
use fallible_iterator::FallibleIterator;

use crate::database::sqlite::utils::DatabaseUtils;
use crate::collection::lyrics::{LyricsDescription, LyricsId};
use crate::collection::lyrics::database_api::LyricsDbApi;
use crate::collection::music::MusicItemId;

pub struct LyricsDb {
    db_utils: DatabaseUtils,
}

impl LyricsDb {
    pub fn new(db_utils: DatabaseUtils) -> Self {
        Self {
            db_utils,
        }
    }
}

impl LyricsDbApi for LyricsDb {
    fn clone_api(&self) -> Box<dyn LyricsDbApi> {
        return Box::new(LyricsDb::new(self.db_utils.clone()));
    }

    fn add_lyrics_item(&self, music_id: MusicItemId, lang_code: &str) -> Result<LyricsId> {
        let context = self.db_utils.lock();
        let query = "INSERT INTO lyrics_items (music_item_id, lang_code) VALUES (?1, ?2)";
        context.connection().execute(&query, params![music_id, lang_code])?;
        Ok(context.connection().last_insert_rowid())
    }

    fn get_lyrics_list(&self, music_id: MusicItemId) -> Result<Vec<LyricsDescription>> {
        let context = self.db_utils.lock();
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
}
