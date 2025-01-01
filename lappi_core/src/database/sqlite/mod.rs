pub mod utils;
pub mod init;
pub mod collection;

use anyhow::Result;
use rusqlite::Connection;
use amina_core::service::Context;

use crate::collection::database_api::CollectionDbApi;
use crate::collection::folders::database_api::FoldersDbApi;
use crate::collection::lyrics::database_api::LyricsDbApi;
use crate::collection::music::database_api::MusicDbApi;
use crate::collection::pictures::database_api::PicturesDbApi;
use crate::collection::playlists::database_api::PlaylistsDbApi;
use crate::collection::tags::database_api::TagsDbApi;
use crate::database::api::{DbExporter, DbImporter};
use crate::debug::configuration::database::Mode;
use crate::debug::Debugger;

use utils::DatabaseUtils;
use collection::folders::FoldersDb;
use collection::pictures::PicturesDb;
use collection::lyrics::LyricsDb;
use collection::music::MusicDb;
use collection::tags::TagsDb;
use collection::playlists::PlaylistsDb;

pub struct SqliteDb {
    db_utils: DatabaseUtils,
    folders_api: Box<dyn FoldersDbApi>,
    pictures_api: Box<dyn PicturesDbApi>,
    music_api: Box<dyn MusicDbApi>,
    tags_api: Box<dyn TagsDbApi>,
    lyrics_api: Box<dyn LyricsDbApi>,
    playlists_api: Box<dyn PlaylistsDbApi>,
}

impl CollectionDbApi for SqliteDb {
    fn get_folders_api(&self) -> Box<dyn FoldersDbApi> {
        self.folders_api.clone_api()
    }

    fn get_lyrics_api(&self) -> Box<dyn LyricsDbApi> {
        self.lyrics_api.clone_api()
    }

    fn get_music_api(&self) -> Box<dyn MusicDbApi> {
        self.music_api.clone_api()
    }

    fn get_tags_api(&self) -> Box<dyn TagsDbApi> {
        self.tags_api.clone_api()
    }

    fn get_pictures_api(&self) -> Box<dyn PicturesDbApi> {
        self.pictures_api.clone_api()
    }

    fn get_playlist(&self) -> Box<dyn PlaylistsDbApi> {
        self.playlists_api.clone_api()
    }

    fn start_batch(&self) {
        self.db_utils.lock().start_batch();
    }

    fn stop_batch(&self) {
        self.db_utils.lock().start_batch();
    }
 
    fn export(&self, exporter: Box<dyn DbExporter>) -> Result<()> {
        let context = self.db_utils.lock();
        let tables_list = self::init::get_tables_list();
        for table_name in tables_list {
            context.export_table(table_name, exporter.as_ref())?;
        }
        Ok(())
    }

    fn import(&self, importer: Box<dyn DbImporter>) -> Result<()> {
        let context = self.db_utils.lock();
        let tables_list = self::init::get_tables_list();
        for table_name in tables_list {
            context.import_table(table_name, importer.as_ref())?;
            log::debug!("Imported table {}", table_name);
        }
        Ok(())
    }
}

pub fn initialize(context: &Context) -> SqliteDb {
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

    let db_utils = DatabaseUtils::new(context, connection);

    SqliteDb {
        db_utils: db_utils.clone(),
        folders_api: Box::new(FoldersDb::new(db_utils.clone())),
        pictures_api: Box::new(PicturesDb::new(db_utils.clone())),
        music_api: Box::new(MusicDb::new(db_utils.clone())),
        tags_api: Box::new(TagsDb::new(db_utils.clone())),
        lyrics_api: Box::new(LyricsDb::new(db_utils.clone())),
        playlists_api: Box::new(PlaylistsDb::new(db_utils.clone())),
    }
}
