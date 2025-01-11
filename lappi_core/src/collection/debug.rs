use crate::debug::Debugger;
use crate::collection::Collection;
use crate::import::collection::basic_csv::BasicCsvCollectionImporter;
use crate::import::collection::basic_yaml::BasicYamlCollectionImporter;

pub fn init() {
    let debugger = crate::context().get_service::<Debugger>();
    let collection = crate::context().get_service::<Collection>();

    if debugger.config().collection.init {
        if collection.is_empty() {
            log::info!("Collection is empty");

            let folder_path = debugger.get_debug_root_workspace().join(&debugger.config().collection.init_folder);
            log::info!("Init folder: {:?}", &folder_path);

            if folder_path.join("items.csv").exists() {
                let importer = BasicCsvCollectionImporter::new(collection);
                importer.import(&folder_path);
            } else if folder_path.join("collection.yaml").exists() {
                let importer = BasicYamlCollectionImporter::new(collection, &folder_path);
                importer.import().unwrap();
            } else {
                panic!("No collection.csv or collection.yaml file found in init folder")
            }
        }
    }
}
