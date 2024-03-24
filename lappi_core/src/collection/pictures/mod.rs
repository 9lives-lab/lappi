use std::path::PathBuf;
use std::sync::Arc;

use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Context, Service};

use crate::collection::database_api::DatabaseApi;
use crate::collection::storage::local::LocalStorage;
use crate::collection::types::{ArtistId, PictureId};
use crate::database::Database;

static FILE_HANDLER_KEY: &str = "lappi.collection.pictures";

#[derive(Clone)]
pub struct PicturesCollection {
    db: Arc<Box<dyn DatabaseApi>>,
    local_storage: Service<LocalStorage>,
}

impl PicturesCollection {
    pub fn initialize(context: &Context) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();
        let database = context.get_service::<Database>();
        let db_api = Arc::new(database.collection());
        let local_storage = context.get_service::<LocalStorage>();

        let pictures = Arc::new(Self {
            db: db_api,
            local_storage,
        });

        register_rpc_handler!(rpc, pictures, "lappi.collection.pictures.copy_to_collection_by_path", copy_to_collection_by_path(file_path: String));
        register_rpc_handler!(rpc, pictures, "lappi.collection.pictures.get_picture_path", get_picture_path(picture_id: PictureId));
        register_rpc_handler!(rpc, pictures, "lappi.collection.pictures.add_picture_to_artist", add_picture_to_artist(picture_id: PictureId, item_id: ArtistId));
        register_rpc_handler!(rpc, pictures, "lappi.collection.pictures.get_pictures_by_artist", get_pictures_by_artist(item_id: ArtistId));

        let pictures_copy = pictures.clone();
        rpc.add_get_file_handler(FILE_HANDLER_KEY, move|path| {
            pictures_copy.get_picture_binary(path)
        });

        return pictures;
    }

    pub fn copy_to_collection_by_path(&self, file_path: String) -> PictureId {
        let file_path = PathBuf::from(file_path);
        let file_extension = file_path.extension().unwrap().to_str().unwrap();
        let picture_id = self.db.add_picture(file_extension);
        let new_file_path = self.get_pictures_storage_path().join(format!("{}.{}", picture_id, file_extension));
        log::debug!("Copying file from {:?} to {:?}", file_path, new_file_path);
        std::fs::copy(file_path, new_file_path).unwrap();
        return picture_id;
    }

    pub fn get_picture_path(&self, picture_id: PictureId) -> String {
        let file_extension = self.db.get_picture_extension(picture_id).unwrap();
        return format!("{}/{}.{}", FILE_HANDLER_KEY, picture_id, file_extension);
    }

    pub fn add_picture_to_artist(&self, picture_id: PictureId, item_id: ArtistId) {
        self.db.add_picture_to_artist(picture_id, item_id).unwrap();
    }

    pub fn get_pictures_by_artist(&self, item_id: ArtistId) -> Vec<PictureId> {
        return self.db.get_pictures_by_artist(item_id).unwrap();
    }

    pub fn get_picture_binary(&self, path: &str) -> Result<Vec<u8>, std::io::Error> {
        let path = self.get_pictures_storage_path().join(path);
        let file_content = std::fs::read(path)?;
        return Ok(file_content);
    }

    fn get_pictures_storage_path(&self) -> PathBuf {
        return self.local_storage.get_internal_storage_folder("pictures");
    }
}
