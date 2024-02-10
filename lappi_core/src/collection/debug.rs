use std::ops::Deref;
use std::sync::Arc;

use amina_core::service::Context;

use crate::collection::Collection;
use crate::import::collection::basic::BasicCollectionImporter;
use crate::debug::Debugger;

pub fn init_collection_from_csv(debugger: &Debugger, collection: Arc<Collection>) {
    log::debug!("Initializing collection from csv");

    let folder_path = debugger.get_debug_root_workspace()
        .join(&debugger.config().collection.init_folder);

    log::debug!("Init folder: {:?}", &folder_path);

    let importer = BasicCollectionImporter::new(collection);
    importer.import(&folder_path);
}

pub fn init(context: &Context, collection: Arc<Collection>) {
    let debugger = context.get_service::<Debugger>();
    if debugger.config().collection.init {
        init_collection_from_csv(debugger.deref(), collection);
    }
}
