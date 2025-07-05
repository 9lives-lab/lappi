pub mod types;
pub mod database_api;

use std::sync::Arc;

use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{AppContext, Service, ServiceApi, ServiceInitializer};

use database_api::InternalFilesDbApi;
use crate::database::Database;
use super::storage::local::LocalStorage;

pub use types::*;

static FILE_HANDLER_KEY: &str = "lappi.collection.internal";

#[derive(Clone)]
pub struct InternalFiles {
    db: Arc<Box<dyn InternalFilesDbApi>>,
    local_storage: Service<LocalStorage>,
}

impl InternalFiles {
    pub fn get_internal_path(&self, file_id: InternalFileId) -> Result<InternalPath> {
        self.db.get_file_path(file_id)
    }

    pub fn get_system_path(&self, file_id: InternalFileId) -> Result<Utf8PathBuf> {
        let internal_path = self.get_internal_path(file_id);
        let mut path = self.local_storage.get_collection_base_path();
        path.push(internal_path?.as_str());
        Ok(path)
    }

    pub fn add_new_file(&self, internal_path: &InternalPath) -> Result<InternalFileId> {
        let file_id = self.db.add_file_path(internal_path)?;

        let path = self.get_system_path(file_id)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::File::create(path)?;
        Ok(file_id)
    }

    pub fn add_and_write_file(&self, src_data: &[u8], internal_path: &InternalPath) -> Result<InternalFileId> {
        let file_id = self.add_new_file(internal_path)?;
        let path = self.get_system_path(file_id)?;
        std::fs::write(path, src_data)?;
        Ok(file_id)
    }

    pub fn add_and_copy_file(&self, src_path: &Utf8Path, internal_path: &InternalPath) -> Result<InternalFileId> {
        let file_id = self.add_new_file(internal_path)?;
        let path = self.get_system_path(file_id)?;
        std::fs::copy(src_path, path)?;
        Ok(file_id)
    }

    pub fn delete_file(&self, file_id: InternalFileId) -> Result<()> {
        let path = self.get_system_path(file_id)?;
        std::fs::remove_file(path)?;
        self.db.delete_file(file_id)?;
        Ok(())
    }

    fn get_binary_rpc_handler(&self, path: &str) -> Result<Vec<u8>, std::io::Error> {
        let mut abs_path = self.local_storage.get_collection_base_path();
        abs_path.push(path);
        let file_content = std::fs::read(abs_path)?;
        Ok(file_content)
    }
}

impl ServiceApi for InternalFiles {

}

impl ServiceInitializer for InternalFiles {
    fn initialize(context: &AppContext) -> Arc<Self> {
        let database = context.get_service::<Database>();
        let db_api = Arc::new(database.get_internal_files_api());
        let local_storage = context.get_service::<LocalStorage>();
        let rpc = context.get_service::<Rpc>();

        let internal_files = Arc::new(Self {
            db: db_api,
            local_storage,
        });

        register_rpc_handler!(rpc, internal_files, "lappi.collection.internal_files.get_internal_path", get_internal_path(file_id: InternalFileId));

        let internal_files_copy = internal_files.clone();
        rpc.add_get_file_handler(FILE_HANDLER_KEY, move|path| {
            internal_files_copy.get_binary_rpc_handler(path)
        });

        return internal_files;
    }
}


