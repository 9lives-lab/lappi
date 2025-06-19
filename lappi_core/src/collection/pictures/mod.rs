pub mod types;
pub mod database_api;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use base64::{Engine as _, engine::general_purpose};
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{AppContext, Service, ServiceApi, ServiceInitializer};

use crate::collection::internal_files::{InternalFiles, InternalPath};
use crate::collection::folders::{FolderId, FoldersCollection};
use crate::database::Database;

use database_api::PicturesDbApi;

pub use types::*;

#[derive(Clone)]
pub struct PicturesCollection {
    db: Arc<Box<dyn PicturesDbApi>>,
    folders: Service<FoldersCollection>,
    internal_files: Service<InternalFiles>,
}

impl PicturesCollection {
    fn add_picture_data_to_collection(&self, folder_id: FolderId, picture_data: &[u8], path: &str) -> Result<PictureId> {
        log::debug!("Add picture to collection. file_name: {:?}", path);

        let file_path = PathBuf::from(path);
        let file_extension = file_path
            .extension().context("File extension not found")?
            .to_str().context("File extension is not valid utf-8")?
            .to_lowercase();

        let mut picture_desc = PictureDesc {
            picture_id: 0,
            folder_id,
            internal_file_id: 0,
            picture_type: PictureType::from_str(&file_extension)?,
        };

        let picture_id = self.db.add_picture_item(&picture_desc)?;
        picture_desc.picture_id = picture_id;

        let internal_path = self.gen_internal_path(picture_id)?;
        let internal_file_id = self.internal_files.add_and_write_file(picture_data, &internal_path)?;
        
        picture_desc.internal_file_id = internal_file_id;
        self.db.update_picture_item(&picture_desc)?;

        Ok(picture_id)
    }

    pub fn add_blob_to_collection(&self, blob: PictureBlob, folder_id: FolderId) -> Result<PictureId> {
        let blob_data = general_purpose::STANDARD.decode(blob.data_base64)?;
        self.add_picture_data_to_collection(folder_id, &blob_data, &blob.file_name)
    }

    pub fn copy_to_collection_by_path(&self, file_path: String, folder_id: FolderId) -> Result<PictureId> {
        let picture_data = std::fs::read(file_path.clone())?;
        self.add_picture_data_to_collection(folder_id, &picture_data, &file_path)
    }

    pub fn delete_picture(&self, picture_id: PictureId) -> Result<()> {
        let picture_desc = self.db.get_picture_descriptor(picture_id)?;
        log::debug!("Deleting picture {:?}", picture_desc);
        self.internal_files.delete_file(picture_desc.internal_file_id)?;
        self.db.delete_picture_item(picture_id)?;
        Ok(())
    }

    pub fn get_pictures_in_folder(&self, folder_id: FolderId) -> Result<Vec<PictureDesc>> {
        self.db.get_pictures_in_folder(folder_id)
    }

    pub fn get_picture_descriptor(&self, picture_id: PictureId) -> Result<PictureDesc> {
        self.db.get_picture_descriptor(picture_id)
    }

    pub fn gen_internal_path(&self, picture_id: PictureId) -> Result<InternalPath> {
        let picture_desc = self.db.get_picture_descriptor(picture_id)?;
        let folder_name = self.folders.get_folder_name(picture_desc.folder_id)?;
        let mut internal_path = self.folders.gen_internal_path(picture_desc.folder_id)?;
        internal_path.push("pictures");
        internal_path.push(format!("{} - {}.{}", &folder_name, picture_desc.picture_id, picture_desc.picture_type.to_str()).as_str());
        Ok(internal_path)
    }
}

impl ServiceApi for PicturesCollection {

}

impl ServiceInitializer for PicturesCollection {
    fn initialize(context: &AppContext) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();
        let database = context.get_service::<Database>();
        let db_api: Arc<Box<dyn PicturesDbApi + 'static>> = Arc::new(database.get_pictures_api());
        let folders = context.get_service::<FoldersCollection>();
        let internal_files = context.get_service::<InternalFiles>();

        let pictures = Arc::new(Self {
            db: db_api,
            folders,
            internal_files,
        });

        register_rpc_handler!(rpc, pictures, "lappi.collection.pictures.add_blob_to_collection", add_blob_to_collection(blob: PictureBlob, folder_id: FolderId));
        register_rpc_handler!(rpc, pictures, "lappi.collection.pictures.copy_to_collection_by_path", copy_to_collection_by_path(file_path: String, folder_id: FolderId));
        register_rpc_handler!(rpc, pictures, "lappi.collection.pictures.delete_picture", delete_picture(picture_id: PictureId));
        register_rpc_handler!(rpc, pictures, "lappi.collection.pictures.get_pictures_in_folder", get_pictures_in_folder(folder_id: FolderId));
        register_rpc_handler!(rpc, pictures, "lappi.collection.pictures.get_picture_descriptor", get_picture_descriptor(picture_id: PictureId));

        return pictures;
    }
}

