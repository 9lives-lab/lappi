pub mod utils;
pub mod init;
pub mod collection;
pub mod playlists;

use std::sync::{Mutex, Arc};

use rusqlite::Connection;
use amina_core::service::Context;

use crate::database::DatabaseApiList;
use crate::database::sqlite::collection::CollectionDbApi;
use crate::database::sqlite::playlists::PlaylistsDbApi;
use crate::debug::configuration::database::Mode;
use crate::debug::Debugger;

pub fn initialize(context: &Context) -> DatabaseApiList {
    let debugger = context.get_service::<Debugger>();

    let connection = match debugger.config().database.sqlite_config.mode {
        Mode::FILE => {
            use crate::platform_api::PlatformApi;

            log::debug!("Initialize SQLite DB in file");

            let platform_api = context.get_service::<PlatformApi>();
            let mut path = platform_api.file_system.get_workspace_dir();
            path.push("db.sql");
            let db_file_exist = path.exists();
            let connection = Connection::open(&path).unwrap();

            if !db_file_exist {
                init::create_tables(&connection).unwrap();
            }

            connection
        },
        Mode::RAM => {
            log::debug!("Initialize SQLite DB in RAM");

            let connection = Connection::open_in_memory().unwrap();
            init::create_tables(&connection).unwrap();

            connection
        }
    };

    let connection = Arc::new(Mutex::new(connection));

    DatabaseApiList {
        collection_api: Box::new(CollectionDbApi::new(context, connection.clone())),
        playlists_api: Box::new(PlaylistsDbApi::new(connection.clone())),
    }
}
