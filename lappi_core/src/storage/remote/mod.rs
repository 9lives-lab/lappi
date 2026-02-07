pub mod adapter;

use std::sync::Arc;

use anyhow::Result;
use amina_core::service::{Context, Service, ServiceApi, ServiceInitializer};

use crate::app_config::AppConfig;
use crate::settings::Settings;
use crate::storage::remote::adapter::file_system::FileSystemFactory;
use crate::storage::remote::adapter::{RemoteStorageAdapter, RemoteStorageFactory, RemoteStorageSettings};

pub struct RemoteStorage {
    storage_available: bool,
    settings: Service<Settings>,
}

impl RemoteStorage {
    pub fn is_available(&self) -> bool {
        return self.storage_available;
    }

    pub fn connect(&self) -> Result<Box<dyn RemoteStorageAdapter>> {
        let factory = FileSystemFactory::new();
        let ftp_url = self.settings.get_string("remote_storage.ftp.url");
        let ftp_user = self.settings.get_string("remote_storage.ftp.user");
        let ftp_password = self.settings.get_string("remote_storage.ftp.password");

        let ftp_settings = RemoteStorageSettings {
            url: ftp_url.get(),
            user: ftp_user.get(),
            password: ftp_password.get(),
            path: "".to_string(),
        };

        factory.connect(&ftp_settings)
    }
}

impl ServiceApi for RemoteStorage {

}

impl ServiceInitializer for RemoteStorage {
    fn initialize(context: &Context) -> Arc<Self> {
        let app_config = context.get_service::<AppConfig>();

        let settings = context.get_service::<Settings>();
        let _ = settings.get_string("remote_storage.ftp.url");
        let _ = settings.get_string("remote_storage.ftp.user");
        let _ = settings.get_string("remote_storage.ftp.password");

        let storage_available = app_config.collection.storage;

        let storage = Arc::new(Self {
            storage_available,
            settings
        });

        return storage;
    }
}

