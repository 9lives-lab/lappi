pub mod types;
pub mod database_api;
pub mod music;
pub mod storage;
pub mod debug;
pub mod pictures;
pub mod folders;

use std::sync::Arc;

use amina_core::events::EventEmitter;
use amina_core::register_event_handler;
use amina_core::service::{ServiceApi, ServiceInitializer, Context, Service};

use crate::database::Database;
use crate::collection::database_api::{DatabaseApi, OnItemsUpdated};
use crate::collection::folders::FoldersView;
use crate::collection::music::MusicCollection;
use crate::collection::pictures::PicturesCollection;
use crate::collection::storage::local::LocalStorage;

pub struct Collection {
    local_storage: Service<LocalStorage>,
    music: Arc<MusicCollection>,
    pictures: Arc<PicturesCollection>,
    folders: Arc<FoldersView>,
    api: Arc<Box<dyn DatabaseApi>>,
}

impl Collection {

    pub fn music(&self) -> &MusicCollection {
        &self.music
    }

    pub fn pictures(&self) -> &PicturesCollection {
        &self.pictures
    }

    pub fn folders(&self) -> &FoldersView {
        &self.folders
    }
    
    pub fn start_batch(&self) {
        self.api.start_batch();
    }

    pub fn stop_batch(&self) {
        self.api.stop_batch();
    }

    fn on_item_updated(&self, _: &OnItemsUpdated) {
        self.folders.update_item();
    }

}

impl ServiceApi for Collection {

    fn start(&self) {
        if self.local_storage.is_available() {
            self.api.import(self.local_storage.get_importer()).unwrap();
        }
    }

    fn stop(&self) {
        if self.local_storage.is_available() {
            self.api.export(self.local_storage.get_exporter()).unwrap();
        }
    }

}

impl ServiceInitializer for Collection {
    fn initialize(context: &Context) -> Arc<Self> {
        let database = context.get_service::<Database>();
        let event_emitter = context.get_service::<EventEmitter>();
        let local_storage = context.get_service::<LocalStorage>();
        let db_api = Arc::new(database.collection());

        let collection = Arc::new(Self {
            local_storage,
            music: MusicCollection::initialize(context),
            pictures: PicturesCollection::initialize(context),
            folders: FoldersView::initialize(context),
            api: db_api.clone(),
        });

        register_event_handler!(event_emitter, collection, on_item_updated);

        debug::init(context, collection.clone());

        return collection;
    }
}
