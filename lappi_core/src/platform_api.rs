use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use amina_core::service::ServiceApi;

use crate::playback::PlayerFactory;

pub trait FileSystemApi: Send + Sync {
    fn get_workspace_dir(&self) -> PathBuf;
}

pub trait PlaybackApi: Send + Sync {
    fn get_platform_player_factories(&self) -> HashMap<String, Box<dyn PlayerFactory>>;
    fn get_defaut_player_factory(&self) -> String;
}

pub struct PlatformApi {
    pub file_system: Arc<dyn FileSystemApi>,
    pub playback: Arc<dyn PlaybackApi>,
}

impl ServiceApi for PlatformApi {

}

