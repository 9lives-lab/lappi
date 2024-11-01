use rusqlite::params;

use crate::collection::playlists::database_api::PlaylistsDbApi;
use crate::collection::playlists::types::{PlaylistDesc, PlaylistId};
use crate::database::api::DbResult;
use crate::database::sqlite::utils::DatabaseUtils;

pub struct PlaylistsDb {
    db_utils: DatabaseUtils,
}

impl PlaylistsDb {
    pub fn new(db_utils: DatabaseUtils) -> Self {
        Self {
            db_utils,
        }
    }
}

impl PlaylistsDbApi for PlaylistsDb {
    fn clone_api(&self) -> Box<dyn PlaylistsDbApi> {
        return Box::new(PlaylistsDb::new(self.db_utils.clone()));
    }

    fn create_classic_playlist(&self, name: &str) -> DbResult<PlaylistId> {
        let context = self.db_utils.lock();
        context.connection().execute(
            "INSERT INTO classic_playlists (name) VALUES (?1)",
            params![name],
        )?;
        let playlist_id = context.connection().last_insert_rowid();
        Ok(playlist_id)
    }

    fn get_classic_playlists(&self) -> DbResult<Vec<PlaylistDesc>> {
        let context = self.db_utils.lock();
        let mut stmt = context.connection().prepare("SELECT id, name FROM classic_playlists")?;
        let rows = stmt.query_map(params![], |row| {
            let id = row.get::<_, i32>(0)?;
            let name = row.get::<_, String>(1)?;
            Ok(PlaylistDesc {
                id: id as PlaylistId,
                name
            })
        })?;

        let list = rows.map(|playlist| playlist.unwrap()).collect();
        Ok(list)
    }
}