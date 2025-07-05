use anyhow::Result;
use camino::Utf8Path;
use rusqlite::params;

use crate::collection::music::MusicItemId;
use crate::collection::pictures::PictureId;
use crate::collection::playlists::database_api::PlaylistsDbApi;
use crate::collection::playlists::types::{PlaylistDesc, PlaylistId, PlaylistItemId};
use crate::database::sqlite::utils::{DatabaseUtils, ProtobufExporter, ProtobufImporter};

pub struct PlaylistsDb {
    db_utils: DatabaseUtils,
}

impl PlaylistsDb {
    pub fn new(db_utils: DatabaseUtils) -> Self {
        Self {
            db_utils,
        }
    }

    pub fn import(&self, base_path: &Utf8Path) -> Result<()> {
        let db_context = self.db_utils.lock();

        let mut importer = ProtobufImporter::create(base_path, "playlists.pb")?;
        while let Some(row) = importer.read_next_row::<crate::proto::collection::PlaylistsRow>()? {
            db_context.connection().execute(
                "INSERT INTO playlists (id, name) VALUES (?1, ?2)",
                params![row.playlist_id, row.name],
            )?;
        }

        let mut importer = ProtobufImporter::create(base_path, "playlist_items.pb")?;
        while let Some(row) = importer.read_next_row::<crate::proto::collection::PlaylistItemsRow>()? {
            db_context.connection().execute(
                "INSERT INTO playlist_items (id, playlist_id, music_item_id) VALUES (?1, ?2, ?3)",
                params![row.playlist_item_id, row.playlist_id, row.music_item_id],
            )?;
        }

        Ok(())
    }

    pub fn export(&self, base_path: &Utf8Path) -> Result<()> {
        let db_context = self.db_utils.lock();
        let mut exporter = ProtobufExporter::create(base_path, "playlists.pb")?;
        let mut stmt = db_context.connection().prepare("SELECT id, name FROM playlists")?;
        let rows = stmt.query_map([], |row| {
            let mut playlist_row = crate::proto::collection::PlaylistsRow::new();
            playlist_row.playlist_id = row.get::<_, i64>(0)?;
            playlist_row.name = row.get::<_, String>(1)?;
            Ok(playlist_row)
        })?;
        for row in rows {
            exporter.write_row(&row?)?;
        }

        let mut exporter = ProtobufExporter::create(base_path, "playlist_items.pb")?;
        let mut stmt = db_context.connection().prepare("SELECT id, playlist_id, music_item_id FROM playlist_items")?;
        let rows = stmt.query_map([], |row| {
            let mut playlist_item_row = crate::proto::collection::PlaylistItemsRow::new();
            playlist_item_row.playlist_item_id = row.get::<_, i64>(0)?;
            playlist_item_row.playlist_id = row.get::<_, i64>(1)?;
            playlist_item_row.music_item_id = row.get::<_, Option<i64>>(2)?;
            Ok(playlist_item_row)
        })?;
        for row in rows {
            exporter.write_row(&row?)?;
        }

        Ok(())
    }
}

impl PlaylistsDbApi for PlaylistsDb {
    fn clone_api(&self) -> Box<dyn PlaylistsDbApi> {
        return Box::new(PlaylistsDb::new(self.db_utils.clone()));
    }

    fn create_playlist(&self, name: &str) -> Result<PlaylistId> {
        let mut context = self.db_utils.lock();
        context.connection().execute(
            "INSERT INTO playlists (name) VALUES (?1)",
            params![name],
        )?;
        let playlist_id = context.connection().last_insert_rowid();
        context.on_playlists_updated();
        log::debug!("Created playlist with id {}", playlist_id);
        Ok(playlist_id)
    }

    fn set_playlist_name(&self, id: PlaylistId, name: &str) -> Result<()> {
        let mut context = self.db_utils.lock();
        context.set_field_value(id, "playlists", "name", name)?;
        context.on_playlists_updated();
        Ok(())
    }

    fn set_playlist_cover(&self, id: PlaylistId, picture_id: Option<PictureId>) -> Result<()> {
        let mut context = self.db_utils.lock();
        context.set_field_value(id, "playlists", "avatar_picture_id", picture_id)?;
        context.on_playlists_updated();
        Ok(())
    }

    fn delete_playlist(&self, id: PlaylistId) -> Result<()> {
        let mut context = self.db_utils.lock();
        context.remove_row("playlists", id)?;
        context.on_playlists_updated();
        Ok(())
    }

    fn get_playlists(&self) -> Result<Vec<PlaylistDesc>> {
        let context = self.db_utils.lock();
        let mut stmt = context.connection().prepare("SELECT id, name, avatar_picture_id FROM playlists")?;
        let rows = stmt.query_map(params![], |row| {
            let id = row.get::<_, i32>(0)?;
            let name = row.get::<_, String>(1)?;
            let avatar_picture_id = row.get:: < _, Option<PictureId>>(2)?;
            Ok(PlaylistDesc {
                id: id as PlaylistId,
                name,
                avatar_picture_id,
            })
        })?;
        Ok(rows.collect::<Result<Vec<_>, _>>()?)
    }

    fn get_playlist_description(&self, playlist_id: PlaylistId) -> Result<PlaylistDesc> {
        let context = self.db_utils.lock();
        let description = context.connection().query_row(
            "SELECT name, avatar_picture_id FROM playlists WHERE id=(?1)",
            params![playlist_id],
            |row| {
                Ok(PlaylistDesc {
                    id: playlist_id,
                    name: row.get:: < _, String>(0)?,
                    avatar_picture_id: row.get:: < _, Option<PictureId>>(1)?,
                })
            },
        )?;
    
        Ok(description)
    }

    fn add_item_to_playlist(&self, playlist_id: PlaylistId, music_item_id: MusicItemId) -> Result<()> {
        let mut context = self.db_utils.lock();
        context.connection().execute(
            "INSERT INTO playlist_items (playlist_id, music_item_id) VALUES (?1, ?2)",
            params![playlist_id, music_item_id],
        )?;
        context.on_playlists_updated();
        Ok(())
    }

    fn delete_item_from_playlist(&self, playlist_id: PlaylistId, music_item_id: MusicItemId) -> Result<()> {
        let mut context = self.db_utils.lock();
        context.connection().execute(
            "DELETE FROM playlist_items WHERE playlist_id=(?1) AND music_item_id=(?2)",
            params![playlist_id, music_item_id],
        )?;
        context.on_playlists_updated();
        Ok(())
    }

    fn get_playlist_items(&self, playlist_id: PlaylistId) -> Result<Vec<(PlaylistItemId, MusicItemId)>> {
        let context = self.db_utils.lock();
        let mut stmt = context.connection().prepare("SELECT id, music_item_id FROM playlist_items WHERE playlist_id=(?1)")?;
        let rows = stmt.query_map(params![playlist_id], |row| {
            Ok((row.get::<_, i32>(0)? as i64, row.get::<_, i32>(1)? as i64))
        })?;
        Ok(rows.collect::<Result<Vec<_>, _>>()?)
    }

    fn get_playlists_for_music_item(&self, music_item_id: MusicItemId) -> Result<Vec<PlaylistId>> {
        let context = self.db_utils.lock();
        let mut stmt = context.connection().prepare("SELECT playlist_id FROM playlist_items WHERE music_item_id=(?1)")?;
        let rows = stmt.query_map(params![music_item_id], |row| {
            Ok(row.get::<_, i32>(0)? as i64)
        })?;
        Ok(rows.collect::<Result<Vec<_>, _>>()?)
    }
}

