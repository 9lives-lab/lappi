pub mod search;

use std::path::Path;
use std::sync::Arc;

use serde::Serialize;
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Context, ServiceApi, ServiceInitializer};

#[derive(Serialize)]
pub struct FilesList {
    pub folders: Vec<String>,
    pub files: Vec<String>,
}

pub struct FileManager {

}

impl FileManager {

    pub fn get_files_list(&self, path: String) -> FilesList {
        let mut folders = Vec::new();
        let mut files = Vec::new();

        let path = Path::new(&path);

        for entry in path.read_dir().expect("read_dir call failed") {
            let entry = entry.expect("read_dir entry failed");
            let path = entry.path();
            let path = path.to_str().unwrap().to_string();

            if entry.file_type().unwrap().is_dir() {
                folders.push(path);
            } else {
                files.push(path);
            }
        }

        return FilesList {
            folders,
            files,
        };
    }

    pub fn get_parent_folder(&self, path: String) -> String {
        let path = Path::new(&path);
        let parent = path.parent();
        match parent {
            None => String::from("/"),
            Some(parent) => parent.to_str().unwrap().to_string(),
        }
    }

    pub fn get_files_in_dir(&self, path: String, recursive: bool) -> Vec<String> {
        let mut files = Vec::new();

        let path = Path::new(&path);

        for entry in path.read_dir().expect("read_dir call failed") {
            let entry = entry.expect("read_dir entry failed");
            let path = entry.path();
            let path = path.to_str().unwrap().to_string();

            if entry.file_type().unwrap().is_file() {
                files.push(path);
            } else if recursive {
                files.append(&mut self.get_files_in_dir(path, recursive));
            }
        }

        return files;
    }

}

impl ServiceApi for FileManager {

}

impl ServiceInitializer for FileManager {
    fn initialize(context: &Context) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();

        let file_manager = Arc::new(Self {

        });

        register_rpc_handler!(rpc, file_manager, "lappi.file_manager.get_files_list", get_files_list(path: String));
        register_rpc_handler!(rpc, file_manager, "lappi.file_manager.get_parent_folder", get_parent_folder(path: String));
        register_rpc_handler!(rpc, file_manager, "lappi.file_manager.get_files_in_dir", get_files_in_dir(path: String, recursive: bool));

        return file_manager;
    }
}
