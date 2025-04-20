use std::path::Path;

use anyhow::Result;
use rusqlite::params;
use fallible_iterator::FallibleIterator;

use crate::database::sqlite::utils::{DatabaseUtils, ProtobufExporter, ProtobufImporter};
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

    pub fn import(&self, base_path: &Path) -> Result<()> {
        let db_context = self.db_utils.lock();
        let mut importer = ProtobufImporter::create(base_path, "lyrics.pb")?;
        while let Some(row) = importer.read_next_row::<crate::proto::collection::LyricsItemsRow>()? {
            db_context.connection().execute(
                "INSERT INTO lyrics_items (id, music_item_id, lang_code) VALUES (?1, ?2, ?3)",
                params![row.lyrics_item_id, row.music_item_id, row.lang_code],
            )?;
        }
        Ok(())
    }

    pub fn export(&self, base_path: &Path) -> Result<()> {
        let db_context = self.db_utils.lock();
        let mut exporter = ProtobufExporter::create(base_path, "lyrics.pb")?;
        let mut stmt = db_context.connection().prepare("SELECT id, music_item_id, lang_code FROM lyrics_items")?;
        let rows = stmt.query_map([], |row| {
            let mut lyrics_row = crate::proto::collection::LyricsItemsRow::new();
            lyrics_row.lyrics_item_id = row.get::<_, i64>(0)?;
            lyrics_row.music_item_id = row.get::<_, i64>(1)?;
            lyrics_row.lang_code = row.get::<_, String>(2)?;
            Ok(lyrics_row)
        })?;
        for row in rows {
            exporter.write_row(&row?)?;
        }
        Ok(())
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
