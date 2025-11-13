pub mod types;
pub mod database_api;

use std::sync::Arc;

use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use walkdir::WalkDir;
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

    pub fn gen_system_path(&self, internal_path: &InternalPath) -> Utf8PathBuf {
        let mut path = self.local_storage.get_collection_base_path();
        path.push(internal_path.as_str());
        path
    }

    pub fn get_system_path(&self, file_id: InternalFileId) -> Result<Utf8PathBuf> {
        let internal_path = self.get_internal_path(file_id)?;
        Ok(self.gen_system_path(&internal_path))
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

    pub fn move_file(&self, file_id: InternalFileId, new_path: &InternalPath) -> Result<()> {
        log::info!("Move. file_id: {}, new_path: {}", file_id, new_path.as_str());
        let current_path = self.get_system_path(file_id)?;
        let new_path_sys = self.gen_system_path(new_path);
        let new_folder = new_path_sys.parent().unwrap();
        if !new_folder.exists() {
            std::fs::create_dir_all(&new_folder)?;
        }
        std::fs::rename(&current_path, &new_path_sys)?;
        self.db.set_file_path(file_id, &new_path)?;
        Ok(())
    }

    pub fn delete_file(&self, file_id: InternalFileId) -> Result<()> {
        let path = self.get_system_path(file_id)?;
        std::fs::remove_file(path)?;
        self.db.delete_file(file_id)?;
        Ok(())
    }

    fn remove_empty_folders_iter(&self) -> Result<i32> {
        let mut folders_num = 0;

        let root_path = self.local_storage.get_collection_base_path();

        for entry in WalkDir::new(&root_path) {
            let entry = entry?;
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            if path.read_dir()?.count() == 0 {
                std::fs::remove_dir(&path)?;
                log::info!("Remove empty folder: '{:?}'", path);
                folders_num += 1;
            }
        }

        Ok(folders_num)
    }

    pub fn remove_empty_folders(&self) -> Result<()> {
        loop {
            let folders_num = self.remove_empty_folders_iter()?;
            if folders_num == 0 {
                break;
            }
        }
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


