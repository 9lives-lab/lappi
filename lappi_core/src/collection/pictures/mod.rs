pub mod types;
pub mod database_api;

use std::path::PathBuf;
use std::sync::Arc;

use base64::{Engine as _, engine::general_purpose};
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Context, Service, ServiceApi, ServiceInitializer};

use crate::collection::storage::local::LocalStorage;
use crate::collection::folders::FolderId;
use crate::database::Database;

use database_api::PicturesDbApi;
pub use types::*;

static FILE_HANDLER_KEY: &str = "lappi.collection.pictures";

#[derive(Clone)]
pub struct PicturesCollection {
    db: Arc<Box<dyn PicturesDbApi>>,
    local_storage: Service<LocalStorage>,
}

impl PicturesCollection {
    pub fn add_blob_to_collection(&self, blob: PictureBlob, folder_id: FolderId) -> PictureId {
        log::debug!("Add blob to collection. file_name: {:?}, file_type: {:?}", blob.file_name, blob.file_type);
        // TODO choose extenstion based on mime type
        let file_path = PathBuf::from(blob.file_name);
        let file_extension = file_path.extension().unwrap().to_str().unwrap();
        let picture_id = self.db.add_picture_item(file_extension, folder_id).unwrap();
        let new_file_path = self.get_pictures_storage_path().join(format!("{}.{}", picture_id, file_extension));
        let blob_data = general_purpose::STANDARD.decode(blob.data_base64).unwrap();
        log::debug!("Writing file to {:?}", new_file_path);
        std::fs::write(new_file_path, blob_data).unwrap();
        return picture_id;
    }

    pub fn copy_to_collection_by_path(&self, file_path: String, folder_id: FolderId) -> PictureId {
        let file_path = PathBuf::from(file_path);
        let file_extension = file_path.extension().unwrap().to_str().unwrap();
        let picture_id = self.db.add_picture_item(file_extension, folder_id).unwrap();
        let new_file_path = self.get_pictures_storage_path().join(format!("{}.{}", picture_id, file_extension));
        log::debug!("Copying file from {:?} to {:?}", file_path, new_file_path);
        std::fs::copy(file_path, new_file_path).unwrap();
        return picture_id;
    }

    pub fn delete_picture(&self, picture_id: PictureId) {
        let file_path = self.get_picture_storage_path(picture_id); 
        log::debug!("Deleting file {:?}", file_path);
        std::fs::remove_file(file_path).unwrap();
        self.db.delete_picture_item(picture_id).unwrap();
    }

    pub fn get_picture_path(&self, picture_id: PictureId) -> String {
        let file_extension = self.db.get_picture_extension(picture_id).unwrap();
        return format!("{}/{}.{}", FILE_HANDLER_KEY, picture_id, file_extension);
    }

    pub fn get_pictures_in_folder(&self, folder_id: FolderId) -> Vec<PictureId> {
        return self.db.get_pictures_in_folder(folder_id).unwrap();
    }

    pub fn get_picture_binary(&self, path: &str) -> Result<Vec<u8>, std::io::Error> {
        let path = self.get_pictures_storage_path().join(path);
        let file_content = std::fs::read(path)?;
        return Ok(file_content);
    }

    fn get_pictures_storage_path(&self) -> PathBuf {
        return self.local_storage.get_internal_storage_folder("pictures");
    }

    fn get_picture_storage_path(&self, picture_id: PictureId) -> PathBuf {
        let file_extension = self.db.get_picture_extension(picture_id).unwrap();
        return self.get_pictures_storage_path().join(format!("{}.{}", picture_id, file_extension));
    }
}

impl ServiceApi for PicturesCollection {

}

impl ServiceInitializer for PicturesCollection {
    fn initialize(context: &Context) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();
        let database = context.get_service::<Database>();
        let db_api = Arc::new(database.get_pictures_api());
        let local_storage = context.get_service::<LocalStorage>();

        let pictures = Arc::new(Self {
            db: db_api,
            local_storage,
        });

        register_rpc_handler!(rpc, pictures, "lappi.collection.pictures.add_blob_to_collection", add_blob_to_collection(blob: PictureBlob, folder_id: FolderId));
        register_rpc_handler!(rpc, pictures, "lappi.collection.pictures.copy_to_collection_by_path", copy_to_collection_by_path(file_path: String, folder_id: FolderId));
        register_rpc_handler!(rpc, pictures, "lappi.collection.pictures.delete_picture", delete_picture(picture_id: PictureId));
        register_rpc_handler!(rpc, pictures, "lappi.collection.pictures.get_picture_path", get_picture_path(picture_id: PictureId));
        register_rpc_handler!(rpc, pictures, "lappi.collection.pictures.get_pictures_in_folder", get_pictures_in_folder(folder_id: FolderId));

        let pictures_copy = pictures.clone();
        rpc.add_get_file_handler(FILE_HANDLER_KEY, move|path| {
            pictures_copy.get_picture_binary(path)
        });

        return pictures;
    }
}

