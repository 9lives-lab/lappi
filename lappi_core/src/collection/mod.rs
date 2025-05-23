pub mod database_api;
pub mod music;
pub mod storage;
pub mod debug;
pub mod pictures;
pub mod folders;
pub mod tags;
pub mod lyrics;
pub mod playlists;

use std::sync::Arc;

use amina_core::service::{ServiceApi, ServiceInitializer, Context, Service};

use crate::database::Database;
use crate::collection::folders::FoldersCollection;
use crate::collection::lyrics::LyricsCollection;
use crate::collection::music::MusicCollection;
use crate::collection::pictures::PicturesCollection;
use crate::collection::playlists::PlaylistsCollection;
use crate::collection::storage::local::LocalStorage;

pub use crate::collection::database_api::OnCollectionUpdated;

pub struct Collection {
    local_storage: Service<LocalStorage>,
    music: Service<MusicCollection>,
    lyrics: Service<LyricsCollection>,
    pictures: Service<PicturesCollection>,
    folders: Service<FoldersCollection>,
    playlists: Service<PlaylistsCollection>,
    db: Service<Database>,
}

impl Collection {
    pub fn music(&self) -> &MusicCollection {
        &self.music
    }

    pub fn lyrics(&self) -> &LyricsCollection {
        &self.lyrics
    }

    pub fn pictures(&self) -> &PicturesCollection {
        &self.pictures
    }

    pub fn folders(&self) -> &FoldersCollection {
        &self.folders
    }

    pub fn playlists(&self) -> &PlaylistsCollection {
        &self.playlists
    }
    
    pub fn start_batch(&self) {
        self.db.start_batch();
    }

    pub fn stop_batch(&self) {
        self.db.stop_batch();
    }

    pub fn is_empty(&self) -> bool {
        self.folders.is_empty()
    }

    pub fn save(&self) {
        if self.local_storage.is_available() {
            let result = self.db.export(&self.local_storage.get_meta_path());
            match result {
                Ok(_) => {
                    log::info!("Collection saved");
                }
                Err(e) => {
                    log::error!("Failed to save collection: {}", e);
                }
                
            }
        }
    }

    pub fn load(&self) {
        if self.local_storage.is_available() {
            let result = self.db.import(&self.local_storage.get_meta_path());
            match result {
                Ok(_) => {
                    log::info!("Collection loaded");
                }
                Err(e) => {
                    log::error!("Failed to load collection: {}", e);
                }
            }
        }
        debug::init();
    }
}

impl ServiceApi for Collection {

    fn start(&self) {
        self.load();
    }

    fn stop(&self) {
        self.save();
    }

}

impl ServiceInitializer for Collection {
    fn initialize(context: &Context) -> Arc<Self> {
        let database = context.get_service::<Database>();
        let local_storage = context.get_service::<LocalStorage>();

        context.init_service::<FoldersCollection>();
        context.init_service::<MusicCollection>();
        context.init_service::<LyricsCollection>();
        context.init_service::<PicturesCollection>();
        context.init_service::<PlaylistsCollection>();

        let collection = Arc::new(Self {
            local_storage,
            music: context.get_service::<MusicCollection>(),
            lyrics: context.get_service::<LyricsCollection>(),
            pictures: context.get_service::<PicturesCollection>(),
            folders: context.get_service::<FoldersCollection>(),
            playlists: context.get_service::<PlaylistsCollection>(),
            db: database,
        });

        return collection;
    }
}
