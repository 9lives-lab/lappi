pub mod types;
pub mod database_api;
pub mod music;
pub mod artists;
pub mod tree;
pub mod storage;
pub mod debug;

use std::sync::Arc;

use amina_core::events::EventEmitter;
use amina_core::register_event_handler;
use amina_core::service::{ServiceApi, ServiceInitializer, Context, Service};

use crate::database::Database;
use crate::collection::database_api::{DatabaseApi, OnItemsUpdated};
use crate::collection::music::MusicCollection;
use crate::collection::artists::ArtistsCollection;
use crate::collection::storage::local::CsvFileDbExporter;
use crate::collection::types::ItemId;
use crate::collection::tree::CollectionView;
use crate::debug::Debugger;
use crate::platform_api::PlatformApi;

pub struct Collection {
    music: Arc<MusicCollection>,
    artists: Arc<ArtistsCollection>,
    view: Arc<CollectionView>,
    api: Arc<Box<dyn DatabaseApi>>,
    platform_api: Service<PlatformApi>,
    storage_available: bool,
}

impl Collection {

    pub fn create_item(&self) -> ItemId {
        return self.api.add_collection_item();
    }

    pub fn add_tag(&self, item_id: ItemId, key: &str, value: &str) {
        self.api.add_tag(item_id, key, value).unwrap();
    }

    pub fn music(&self) -> &MusicCollection {
        &self.music
    }

    pub fn artists(&self) -> &ArtistsCollection {
        &self.artists
    }

    pub fn view(&self) -> &CollectionView {
        &self.view
    }
    
    pub fn start_batch(&self) {
        self.api.start_batch();
    }

    pub fn stop_batch(&self) {
        self.api.stop_batch();
    }

    fn on_item_updated(&self, event: &OnItemsUpdated) {
        for item in &event.items {
            self.view.update_item(*item);
        }
    }

}

impl ServiceApi for Collection {

    fn start(&self) {
        if self.storage_available {
            let ws_path = self.platform_api.file_system.get_workspace_dir();
            let importer = Box::new(storage::local::CsvFileDbImporter::create(ws_path));
            self.api.import(importer).unwrap();
        }
    }

    fn stop(&self) {
        if self.storage_available {
            let ws_path = self.platform_api.file_system.get_workspace_dir();
            let exporter = Box::new(CsvFileDbExporter::create(ws_path));
            self.api.export(exporter).unwrap();
        }
    }

}

impl ServiceInitializer for Collection {
    fn initialize(context: &Context) -> Arc<Self> {
        let database = context.get_service::<Database>();
        let event_emitter = context.get_service::<EventEmitter>();
        let platform_api = context.get_service::<PlatformApi>();
        let debugger = context.get_service::<Debugger>();

        let db_api = Arc::new(database.collection());

        let collection = Arc::new(Self {
            music: MusicCollection::initialize(context),
            artists: ArtistsCollection::initialize(context),
            view: CollectionView::initialize(context),
            api: db_api.clone(),
            platform_api,
            storage_available: debugger.config().collection.storage,
        });

        register_event_handler!(event_emitter, collection, on_item_updated);

        debug::init(context, collection.clone());

        return collection;
    }
}
