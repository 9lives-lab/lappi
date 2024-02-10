pub mod configuration;

use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

use amina_core::service::{Context, ServiceApi, ServiceInitializer};

use crate::debug::configuration::DebugConfig;
use crate::platform_api::PlatformApi;

pub struct Debugger {
    debug_config: DebugConfig,
    debug_root_workspace: PathBuf,
}

impl Debugger {

    pub fn config(&self) -> &DebugConfig {
        &self.debug_config
    }

    pub fn get_debug_root_workspace(&self) -> PathBuf {
        self.debug_root_workspace.clone()
    }

}

impl ServiceApi for Debugger {

}

impl ServiceInitializer for Debugger {
    fn initialize(context: &Context) -> Arc<Self> {
        let platform = context.get_service::<PlatformApi>();
        let workspace_dir = platform.file_system.get_workspace_dir();
        let debug_root_workspace = workspace_dir.parent().unwrap().to_path_buf();

        let mut config_file_path = platform.file_system.get_workspace_dir();
        config_file_path.push("debug_config.yaml");
        let debug_config: DebugConfig = serde_yaml::from_reader(File::open(&config_file_path).unwrap()).unwrap();

        Arc::new(Self {
            debug_config,
            debug_root_workspace,
        })
    }
}
