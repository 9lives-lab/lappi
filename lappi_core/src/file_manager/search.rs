use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use camino::Utf8Path;
use serde::Serialize;
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Context, ServiceApi, ServiceInitializer};

use crate::collection::tags::TagValue;
use crate::metadata;

#[derive(Serialize)]
pub struct FileDescription {
    pub media_type: String,
    pub tags: HashMap<String, TagValue>,
}

pub struct FilesExplorer {

}

impl FilesExplorer {

    pub fn get_file_description(&self, path: String) -> Result<FileDescription> {
        let path = Utf8Path::new(&path);

        let metadata = metadata::read_from_path(path)?;
        Ok(match metadata {
            Some(metadata) => {
                FileDescription {
                    media_type: metadata.media_type,
                    tags: metadata.tags.get_tags_map().clone(),
                }
            },
            None => {
                FileDescription {
                    media_type: String::from("unknown"),
                    tags: HashMap::new(),
                }
            }
        })
    }
}

impl ServiceApi for FilesExplorer {

}

impl ServiceInitializer for FilesExplorer {
    fn initialize(context: &Context) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();

        let files_explorer = Arc::new(Self {

        });

        register_rpc_handler!(rpc, files_explorer, "lappi.files_explorer.get_file_description", get_file_description(path: String));

        return files_explorer;
    }
}
