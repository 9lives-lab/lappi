use anyhow::Result;

use crate::collection::internal_files::{InternalFileId, InternalPath};

pub trait InternalFilesDbApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn InternalFilesDbApi>;

    fn add_file_path(&self, path: &InternalPath) -> Result<InternalFileId>;
    fn get_file_path(&self, file_id: InternalFileId) -> Result<InternalPath>;
    fn delete_file(&self, file_id: InternalFileId) -> Result<()>;
}

