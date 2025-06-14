pub mod types;
pub mod database_api;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Context, Service, ServiceApi, ServiceInitializer};

use database_api::InternalFilesDbApi;
use crate::database::Database;
use super::storage::local::LocalStorage;

pub use types::*;

#[derive(Clone)]
pub struct InternalFiles {
    db: Arc<Box<dyn InternalFilesDbApi>>,
    local_storage: Service<LocalStorage>,
}

impl InternalFiles {
    pub fn import_file(&self, src_path: &Path, internal_path: &InternalPath) -> Result<InternalFileId> {
        let dst_path = self.get_storage_abs_path(internal_path);
        if let Some(parent) = dst_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::copy(src_path, dst_path)?;
        let file_id = self.db.add_file_path(internal_path)?;
        return Ok(file_id);
    }

    pub fn get_file_path(&self, file_id: InternalFileId) -> InternalPath {
        return self.db.get_file_path(file_id).unwrap();
    }

    pub fn delete_file(&self, file_id: InternalFileId) -> Result<()> {
        let path = self.db.get_file_path(file_id)?;
        let abs_path = self.get_storage_abs_path(&path);
        std::fs::remove_file(abs_path)?;
        return Ok(());
    }

    pub fn get_storage_abs_path(&self, internal_path: &InternalPath) -> PathBuf {
        let mut path = self.local_storage.get_collection_base_path();
        path.push(internal_path.as_str());
        return path;
    }
}

impl ServiceApi for InternalFiles {

}

impl ServiceInitializer for InternalFiles {
    fn initialize(context: &Context) -> Arc<Self> {
        let database = context.get_service::<Database>();
        let db_api = Arc::new(database.get_internal_files_api());
        let local_storage = context.get_service::<LocalStorage>();
        let rpc = context.get_service::<Rpc>();

        let internal_files = Arc::new(Self {
            db: db_api,
            local_storage,
        });

        register_rpc_handler!(rpc, internal_files, "lappi.collection.internal_files.get_file_path", get_file_path(file_id: InternalFileId));

        return internal_files;
    }
}


