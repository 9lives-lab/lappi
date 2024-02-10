use std::sync::{Mutex, Arc};

use rusqlite::{params, Connection};

use crate::database_api::DbResult;
use crate::playlists::database_api::DatabaseApi;
use crate::playlists::types::{PlaylistDesc, PlaylistId};

pub struct PlaylistsDbApi {
    connection: Arc<Mutex<Connection>>,
}

impl PlaylistsDbApi {
    pub fn new(connection: Arc<Mutex<Connection>>) -> Self {
        Self {
            connection
        }
    }
}

impl DatabaseApi for PlaylistsDbApi {

    fn clone_api(&self) -> Box<dyn DatabaseApi> {
        Box::new(Self::new(self.connection.clone()))
    }

    fn create_classic_playlist(&self, name: &str) -> DbResult<PlaylistId> {
        let conn = self.connection.lock().unwrap();
        conn.execute(
            "INSERT INTO classic_playlists (name) VALUES (?1)",
            params![name],
        )?;
        let playlist_id = conn.last_insert_rowid();
        Ok(playlist_id)
    }

    fn get_classic_playlists(&self) -> DbResult<Vec<PlaylistDesc>> {
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name FROM classic_playlists")?;
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
