pub mod search;

use std::sync::Arc;

use anyhow::Result;
use camino::Utf8Path;
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

    pub fn get_files_list(&self, path: String) -> Result<FilesList> {
        let mut folders = Vec::new();
        let mut files = Vec::new();

        let path = Utf8Path::new(&path);

        for entry in path.read_dir_utf8()? {
            match entry {
                Ok(entry) => {
                    let path = entry.path().to_string();

                    match entry.file_type() {
                        Ok(file_type) => {
                            if file_type.is_dir() {
                                folders.push(path);
                            } else {
                                files.push(path);
                            }
                        },
                        Err(e) => {
                            log::error!("Failed to get file type: {}", e);
                        }
                    }
                },
                Err(e) => {
                    log::error!("Failed to read dir entry: {}", e);
                }
            }
        }

        Ok(FilesList {
            folders,
            files,
        })
    }

    pub fn get_parent_folder(&self, path: String) -> String {
        let path = Utf8Path::new(&path);
        let parent = path.parent();
        match parent {
            None => String::from("/"),
            Some(parent) => parent.to_string(),
        }
    }

    pub fn get_files_in_dir(&self, path: String, recursive: bool) -> Result<Vec<String>> {
        let mut files = Vec::new();

        let path = Utf8Path::new(&path);

        for entry in path.read_dir_utf8()? {
            match entry {
                Ok(entry) => {
                    let path = entry.path().to_string();

                    match entry.file_type() {
                        Ok(file_type) => {
                            if file_type.is_file() {
                                files.push(path);
                            } else if recursive {
                                files.append(&mut self.get_files_in_dir(path, recursive)?);
                            }
                        },
                        Err(e) => {
                            log::error!("Failed to get file type: {}", e);
                        }
                    }
                },
                Err(e) => {
                    log::error!("Failed to read directory entry: {}", e);
                }
            }
        }

        Ok(files)
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
