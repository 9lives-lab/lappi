use anyhow::Result;

use crate::app_config::AppConfig;
use crate::collection::Collection;
use crate::platform_api::PlatformApi;
use crate::import::collection::basic_csv::BasicCsvCollectionImporter;
use crate::import::collection::basic_yaml::BasicYamlCollectionImporter;

pub fn init() -> Result<()> {
    let platform_api = crate::context().get_service::<PlatformApi>();
    let app_config = crate::context().get_service::<AppConfig>();
    let collection = crate::context().get_service::<Collection>();

    if app_config.collection.init {
        if collection.is_empty() {
            log::info!("Collection is empty");

            let folder_path = platform_api.file_system.get_workspace_dir().join(&app_config.collection.init_folder);
            log::info!("Init folder: {:?}", &folder_path);

            if folder_path.join("items.csv").exists() {
                let importer = BasicCsvCollectionImporter::new();
                importer.import(&folder_path)?;
            } else if folder_path.join("collection.yaml").exists() {
                let importer = BasicYamlCollectionImporter::new(&folder_path);
                importer.import()?;
            } else {
                panic!("No collection.csv or collection.yaml file found in init folder")
            }
        }
    }

    Ok(())
}
