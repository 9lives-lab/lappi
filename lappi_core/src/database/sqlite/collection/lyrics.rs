use anyhow::Result;
use camino::Utf8Path;
use rusqlite::params;
use fallible_iterator::FallibleIterator;

use crate::collection::internal_files::InternalFileId;
use crate::database::sqlite::utils::{DatabaseUtils, ProtobufExporter, ProtobufImporter};
use crate::collection::lyrics::{LyricsDesc, LyricsId};
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

    pub fn import(&self, base_path: &Utf8Path) -> Result<()> {
        let db_context = self.db_utils.lock();
        let mut importer = ProtobufImporter::create(base_path, "lyrics.pb")?;
        while let Some(row) = importer.read_next_row::<crate::proto::collection::LyricsItemsRow>()? {
            db_context.connection().execute(
                "INSERT INTO lyrics_items (id, music_item_id, lyrics_tag, internal_file_id) VALUES (?1, ?2, ?3, ?4)",
                params![row.lyrics_item_id, row.music_item_id, row.lyrics_tag, row.internal_file_id],
            )?;
        }
        Ok(())
    }

    pub fn export(&self, base_path: &Utf8Path) -> Result<()> {
        let db_context = self.db_utils.lock();
        let mut exporter = ProtobufExporter::create(base_path, "lyrics.pb")?;
        let mut stmt = db_context.connection().prepare("SELECT id, music_item_id, lyrics_tag, internal_file_id FROM lyrics_items")?;
        let rows = stmt.query_map([], |row| {
            let mut lyrics_row = crate::proto::collection::LyricsItemsRow::new();
            lyrics_row.lyrics_item_id = row.get(0)?;
            lyrics_row.music_item_id = row.get(1)?;
            lyrics_row.lyrics_tag = row.get(2)?;
            lyrics_row.internal_file_id = row.get(3)?;

            Ok(lyrics_row)
        })?;
        exporter.write_rows(rows)?;
        Ok(())
    }
}

impl LyricsDbApi for LyricsDb {
    fn clone_api(&self) -> Box<dyn LyricsDbApi> {
        return Box::new(LyricsDb::new(self.db_utils.clone()));
    }

    fn add_lyrics_item(&self, music_id: MusicItemId, lyrics_tag: &str, internal_file_id: InternalFileId) -> Result<LyricsId> {
        let context = self.db_utils.lock();
        let query = "INSERT INTO lyrics_items (music_item_id, lyrics_tag, internal_file_id) VALUES (?1, ?2, ?3)";
        context.connection().execute(&query, params![music_id, lyrics_tag, internal_file_id])?;
        Ok(context.connection().last_insert_rowid())
    }

    fn get_lyrics_descriptor(&self, lyrics_id: LyricsId) -> Result<LyricsDesc> {
        let context = self.db_utils.lock();
        let mut stmt = context.connection().prepare("SELECT lyrics_tag, internal_file_id FROM lyrics_items WHERE id = ?1")?;
        let row = stmt.query_row(params![lyrics_id], |row| {
            Ok(LyricsDesc {
                lyrics_id,
                lyrics_tag: row.get(0)?,
                internal_file_id: row.get(1)?,
            })
        })?;
        Ok(row)
    }

    fn get_lyrics_list(&self, music_id: MusicItemId) -> Result<Vec<LyricsDesc>> {
        let context = self.db_utils.lock();
        let mut stmt = context.connection().prepare("SELECT id, lyrics_tag, internal_file_id FROM lyrics_items WHERE music_item_id=(?1)")?;
        let rows = stmt.query([music_id])?;
        let rows = rows.map(|row| {
            Ok(LyricsDesc {
                lyrics_id: row.get(0)?,
                lyrics_tag: row.get(1)?,
                internal_file_id: row.get(2)?,
            })
        });
        Ok(rows.collect()?)
    }
}
