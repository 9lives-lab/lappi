pub mod types;
pub mod database_api;

use std::path::Path;
use std::sync::Arc;

use anyhow::{Error, Result};
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Context, Service, ServiceApi, ServiceInitializer};

use crate::collection::music::{MusicCollection, MusicItemId};
use crate::database::Database;
use crate::collection::internal_files::InternalFiles;

use database_api::MusicSourcesDbApi;

pub use types::*;

pub struct MusicSourcesCollection {
    music_sources_db: Arc<Box<dyn MusicSourcesDbApi>>,
    internal_files: Service<InternalFiles>,
    music: Service<MusicCollection>
}

impl MusicSourcesCollection {
    pub fn import_music_file(&self, item_id: MusicItemId, src_path: &Path) -> Result<()> {
        self.delete_music_file(item_id);

        if !src_path.exists() {
            return Err(Error::msg("Path does not exist"));
        }

        if !src_path.is_file() {
            return Err(Error::msg("Path is not a file"));
        }

        let extention = src_path.extension().ok_or_else(|| Error::msg("File has no extention"))?;
        let extention = extention.to_str().ok_or_else(|| Error::msg("File extention is not valid utf8"))?;
        let extention = extention.to_lowercase();

        let file_type = match extention.as_str() {
            "mp3" => MusicFileType::MP3,
            _ => return Err(Error::msg("Unsupported file extention")),
            
        };

        let template: String = "{file_name}.".to_string() + extention.as_str();
        let internal_path = self.music.gen_internal_path(item_id, &template);

        let file_id = self.internal_files.add_and_copy_file(src_path, &internal_path)?;

        let file_desc = MusicFileDesc {
            music_item_id: item_id,
            internal_file_id: file_id,
            file_type,
        };
        self.music_sources_db.add_music_file(&file_desc)?;

        Ok(())
    }

    pub fn get_music_file(&self, item_id: MusicItemId) -> Option<MusicFileDesc> {
        self.music_sources_db.get_music_file(item_id).unwrap()
    }

    pub fn delete_music_file(&self, item_id: MusicItemId) {
        if let Some(file_desc) = self.music_sources_db.get_music_file(item_id).unwrap() {
            self.internal_files.delete_file(file_desc.internal_file_id).unwrap();
            self.music_sources_db.delete_music_file(item_id).unwrap();
        }
    }

    pub fn add_music_link(&self, item_id: MusicItemId, link_type: MusicLinkType, link: String) {
        self.music_sources_db.add_music_link(item_id, link_type, &link).unwrap();
    }

    pub fn set_music_link(&self, link_id: MusicLinkId, link: String) {
        self.music_sources_db.set_music_link(link_id, &link).unwrap();
    }

    pub fn get_music_links(&self, item_id: MusicItemId) -> Vec<MusicLinkDesc> {
        self.music_sources_db.get_music_links(item_id).unwrap()
    }

    pub fn delete_music_link(&self, link_id: MusicLinkId) {
        self.music_sources_db.delete_music_link(link_id).unwrap();
    }
}

impl ServiceApi for MusicSourcesCollection {

}

impl ServiceInitializer for MusicSourcesCollection {
    fn initialize(context: &Context) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();
        let database = context.get_service::<Database>();

        let music_sources = Arc::new(Self {
            music_sources_db: Arc::new(database.get_music_sources_api()),
            internal_files: context.get_service::<InternalFiles>(),
            music: context.get_service::<MusicCollection>(),
        });

        register_rpc_handler!(rpc, music_sources, "lappi.collection.music_sources.get_music_file", get_music_file(item_id: MusicItemId));
        register_rpc_handler!(rpc, music_sources, "lappi.collection.music_sources.delete_music_file", delete_music_file(item_id: MusicItemId));
        register_rpc_handler!(rpc, music_sources, "lappi.collection.music_sources.add_music_link", add_music_link(item_id: MusicItemId, link_type: MusicLinkType, link: String));
        register_rpc_handler!(rpc, music_sources, "lappi.collection.music_sources.set_music_link", set_music_link(link_id: MusicLinkId, link: String));
        register_rpc_handler!(rpc, music_sources, "lappi.collection.music_sources.get_music_links", get_music_links(item_id: MusicItemId));
        register_rpc_handler!(rpc, music_sources, "lappi.collection.music_sources.delete_music_link", delete_music_link(link_id: MusicLinkId));

        return music_sources;
    }
}
