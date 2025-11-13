use std::fs;
use std::sync::Arc;

use camino::Utf8PathBuf;
use amina_core::service::{AppContext, Service, ServiceApi, ServiceInitializer};

use crate::platform_api::PlatformApi;

pub struct Workspace {
    #[allow(dead_code)]
    platform_api: Service<PlatformApi>,
    temp_folder_path: Utf8PathBuf,
}

impl Workspace {
    pub fn get_temp_dir(&self) -> Utf8PathBuf {
        self.temp_folder_path.clone()
    }
}

impl ServiceApi for Workspace {

}

impl ServiceInitializer for Workspace {
    fn initialize(context: &AppContext) -> Arc<Self> {
        let platform_api = context.get_service::<PlatformApi>();

        let temp_folder_path = platform_api.file_system.get_workspace_dir().join("temp");
        if !temp_folder_path.exists() {
            fs::create_dir(&temp_folder_path).unwrap();
        }

        let workspace = Arc::new(Workspace {
            platform_api,
            temp_folder_path,
        });

        return workspace;
    }
}

