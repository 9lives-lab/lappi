use std::sync::Arc;
use std::fs::File;

use serde::Deserialize;
use amina_core::service::{Context, ServiceApi, ServiceInitializer};

use crate::platform_api::PlatformApi;

pub mod database {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub enum Mode {
        RAM,
        FILE
    }

    #[derive(Deserialize)]
    pub struct SqliteConfig {
        pub mode: Mode,
    }

    #[derive(Deserialize)]
    pub struct Config {
        pub sqlite_config: SqliteConfig,
    }

}

pub mod collection {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Config {
        pub init: bool,
        pub init_folder: String,
        pub storage: bool,
    }

}

pub mod web_server {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Config {
        pub port: u16,
        pub static_files_path: String,
    }

}

#[derive(Deserialize)]
pub struct AppConfig {
    pub database: self::database::Config,
    pub collection: self::collection::Config,
    pub web_server: self::web_server::Config,
}

impl ServiceApi for AppConfig {

}

impl ServiceInitializer for AppConfig {
    fn initialize(context: &Context) -> Arc<Self> {
        let platform = context.get_service::<PlatformApi>();
        let mut config_file_path = platform.file_system.get_workspace_dir();
        config_file_path.push("app_config.yaml");
        let app_config: AppConfig = serde_yaml::from_reader(File::open(&config_file_path).unwrap()).unwrap();
        Arc::new(app_config)
    }
}

