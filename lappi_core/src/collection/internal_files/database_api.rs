use anyhow::Result;

use crate::collection::internal_files::{InternalFileId, InternalPath, FileHash};

pub trait InternalFilesDbApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn InternalFilesDbApi>;

    fn add_file_path(&self, path: &InternalPath) -> Result<InternalFileId>;
    fn get_file_path(&self, file_id: InternalFileId) -> Result<InternalPath>;
    fn get_file_hash(&self, file_id: InternalFileId) -> Result<FileHash>;
    fn set_file_path(&self, file_id: InternalFileId, path: &InternalPath) -> Result<()>;
    fn set_file_hash(&self, file_id: InternalFileId, hash: &FileHash) -> Result<()>;
    fn delete_file(&self, file_id: InternalFileId) -> Result<()>;
    fn get_all_files(&self) -> Result<Vec<InternalFileId>>;
}

