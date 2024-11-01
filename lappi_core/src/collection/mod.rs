pub mod database_api;
pub mod music;
pub mod storage;
pub mod debug;
pub mod pictures;
pub mod folders;
pub mod lyrics;
pub mod playlists;

use std::sync::Arc;

use amina_core::service::{ServiceApi, ServiceInitializer, Context, Service};

use crate::database::Database;
use crate::collection::folders::FoldersCollection;
use crate::collection::lyrics::LyricsCollection;
use crate::collection::music::MusicCollection;
use crate::collection::pictures::PicturesCollection;
use crate::collection::storage::local::LocalStorage;

pub use crate::collection::database_api::OnCollectionUpdated;

pub struct Collection {
    local_storage: Service<LocalStorage>,
    music: Arc<MusicCollection>,
    lyrics: Arc<LyricsCollection>,
    pictures: Arc<PicturesCollection>,
    folders: Arc<FoldersCollection>,
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
    
    pub fn start_batch(&self) {
        self.db.start_batch();
    }

    pub fn stop_batch(&self) {
        self.db.stop_batch();
    }
}

impl ServiceApi for Collection {

    fn start(&self) {
        if self.local_storage.is_available() {
            self.db.import(self.local_storage.get_importer()).unwrap();
        }
    }

    fn stop(&self) {
        if self.local_storage.is_available() {
            self.db.export(self.local_storage.get_exporter()).unwrap();
        }
    }

}

impl ServiceInitializer for Collection {
    fn initialize(context: &Context) -> Arc<Self> {
        let database = context.get_service::<Database>();
        let local_storage = context.get_service::<LocalStorage>();

        let collection = Arc::new(Self {
            local_storage,
            music: MusicCollection::initialize(context),
            lyrics: LyricsCollection::initialize(context),
            pictures: PicturesCollection::initialize(context),
            folders: FoldersCollection::initialize(context),
            db: database,
        });

        debug::init(context, collection.clone());

        return collection;
    }
}
