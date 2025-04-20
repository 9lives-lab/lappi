pub mod utils;
pub mod init;
pub mod collection;

use std::path::Path;

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
use crate::app_config::{self, AppConfig};

use utils::DatabaseUtils;
use collection::folders::FoldersDb;
use collection::pictures::PicturesDb;
use collection::lyrics::LyricsDb;
use collection::music::MusicDb;
use collection::tags::TagsDb;
use collection::playlists::PlaylistsDb;

pub struct SqliteDb {
    db_utils: DatabaseUtils,
    folders_api: Box<FoldersDb>,
    pictures_api: Box<PicturesDb>,
    music_api: Box<MusicDb>,
    tags_api: Box<TagsDb>,
    lyrics_api: Box<LyricsDb>,
    playlists_api: Box<PlaylistsDb>,
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
        self.db_utils.lock().stop_batch();
    }
 
    fn export(&self, base_path: &Path) -> Result<()> {
        std::fs::create_dir_all(base_path)?; 
        self.folders_api.export(base_path)?;
        self.music_api.export(base_path)?;
        self.tags_api.export(base_path)?;
        self.pictures_api.export(base_path)?;
        self.lyrics_api.export(base_path)?;
        self.playlists_api.export(base_path)?;
        Ok(())
    }

    fn import(&self, base_path: &Path) -> Result<()> {
        self.folders_api.import(base_path)?;
        self.music_api.import(base_path)?;
        self.tags_api.import(base_path)?;
        self.pictures_api.import(base_path)?;
        self.lyrics_api.import(base_path)?;
        self.playlists_api.import(base_path)?;
        Ok(())
    }
}

pub fn initialize(context: &Context) -> SqliteDb {
    let app_config = context.get_service::<AppConfig>();

    let connection = match app_config.database.sqlite_config.mode {
        app_config::database::Mode::FILE => {
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
        app_config::database::Mode::RAM => {
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
