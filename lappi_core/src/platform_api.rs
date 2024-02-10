use std::path::PathBuf;
use std::sync::Arc;

use amina_core::service::ServiceApi;

pub trait FileSystemApi: Send + Sync {
    fn get_workspace_dir(&self) -> PathBuf;
}

pub struct PlatformApi {
    pub file_system: Arc<dyn FileSystemApi>,
}

impl ServiceApi for PlatformApi {

}

