use std::path::PathBuf;
use std::sync::Arc;

use amina_core::service::ServiceApi;

use crate::playback::Player;

pub trait FileSystemApi: Send + Sync {
    fn get_workspace_dir(&self) -> PathBuf;
}

pub trait PlayerApi: Send + Sync {
    fn create_platform_player(&self) -> Box<dyn Player>;
}

pub struct PlatformApi {
    pub file_system: Arc<dyn FileSystemApi>,
    pub player: Arc<dyn PlayerApi>,
}

impl ServiceApi for PlatformApi {

}

