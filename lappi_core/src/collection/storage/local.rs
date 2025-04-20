use std::path::PathBuf;
use std::sync::Arc;

use amina_core::service::{Context, ServiceApi, ServiceInitializer};

use crate::app_config::AppConfig;
use crate::platform_api::PlatformApi;


pub struct LocalStorage {
    storage_path: PathBuf,
    storage_available: bool,
}

impl LocalStorage {
    pub fn is_available(&self) -> bool {
        return self.storage_available;
    }

    pub fn get_collection_base_path(&self) -> PathBuf {
        return self.storage_path.clone();
    }

    pub fn get_internal_storage_path(&self) -> PathBuf {
        return self.get_collection_base_path().join(".lappi");
    }

    pub fn get_internal_storage_folder(&self, folder_name: &str) -> PathBuf {
        return self.get_internal_storage_path().join(folder_name);
    }

    pub fn get_meta_path(&self) -> PathBuf {
        return self.get_internal_storage_folder("meta");
    }
}

impl ServiceApi for LocalStorage {

}

impl ServiceInitializer for LocalStorage {
    fn initialize(context: &Context) -> Arc<Self> {
        let platform_api = context.get_service::<PlatformApi>();
        let app_config = context.get_service::<AppConfig>();

        let storage_available = app_config.collection.storage;
        let storage_path = platform_api.file_system.get_workspace_dir().join("collection");

        let storage = Arc::new(Self {
            storage_path,
            storage_available,
        });

        return storage;
    }
}
