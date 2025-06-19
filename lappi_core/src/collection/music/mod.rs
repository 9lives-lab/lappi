pub mod types;
pub mod database_api;

use std::sync::Arc;

use anyhow::Result;
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{AppContext, Service, ServiceApi, ServiceInitializer};

use crate::database::Database;
use crate::collection::internal_files::InternalPath;

use super::folders::{FolderId, FoldersCollection};
use super::pictures::PictureId;
use super::tags::database_api::TagsDbApi;
use super::tags::{Tag, TagValue};

use database_api::MusicDbApi;

pub use types::*;

pub struct MusicCollection {
    music_db: Arc<Box<dyn MusicDbApi>>,
    tags_db: Arc<Box<dyn TagsDbApi>>,
    folders: Service<FoldersCollection>,
}

impl MusicCollection {
    pub fn create_item(&self, name: String, folder_id: FolderId) -> Result<MusicItemId> {
        self.music_db.add_music_item(&name, folder_id)
    }

    pub fn set_item_name(&self, item_id: MusicItemId, name: String) -> Result<()> {
        self.music_db.set_item_name(item_id, &name)
    }

    pub fn get_item_description(&self, item_id: MusicItemId) -> Result<MusicItemDesc> {
        self.music_db.get_music_item_description(item_id)
    }

    pub fn get_caption_tag(&self, item_id: MusicItemId) -> Result<Option<Tag>> {
        self.get_tag(item_id, "track")
    }

    pub fn get_item_caption(&self, item_id: MusicItemId) -> Result<String> {
        if let Some(tag) = self.get_caption_tag(item_id)? {
            Ok(format!("{} - {}", tag.get_key(), tag.to_string()))
        } else {
            Ok("".to_string())
        }
    }

    pub fn get_item_cover(&self, item_id: MusicItemId) -> Result<Option<PictureId>> {
        self.folders.find_folder_cover(self.get_item_description(item_id)?.folder_id)
    }

    pub fn set_tag(&self, item_id: MusicItemId, tag_name: String, tag_value: TagValue) -> Result<()> {
        log::debug!("set_tag: item_id: {}, tag_name: {}, tag_value: {:?}", item_id, tag_name, tag_value);
        self.tags_db.set_add_item_tag(item_id, tag_name.as_str(), &tag_value)
    }

    pub fn get_tags(&self, item_id: MusicItemId) -> Result<Vec<Tag>> {
        self.tags_db.get_item_tags(item_id)
    }

    pub fn get_inherited_tags(&self, item_id: MusicItemId) -> Result<Vec<Tag>> {
        let mut tags = vec![];

        let decription = self.get_item_description(item_id)?;
        let folder_id = decription.folder_id;

        tags.extend(self.folders.get_tags(folder_id)?);
        tags.extend(self.folders.get_inherited_tags(folder_id)?);

        return Ok(tags);
    }

    pub fn get_tag(&self, item_id: MusicItemId, tag_name: &str) -> Result<Option<Tag>> {
        let mut tags = vec![];

        tags.extend(self.get_tags(item_id)?);
        tags.extend(self.get_inherited_tags(item_id)?);

        for tag in tags {
            if tag.get_key() == tag_name {
                return Ok(Some(tag));
            }
        }

        return Ok(None);
    }

    pub fn delete_tag(&self, item_id: MusicItemId, tag_name: String) -> Result<()> {
        self.tags_db.delete_item_tag(item_id, &tag_name)
    }

    pub fn gen_internal_path(&self, item_id: MusicItemId, template: &str) -> Result<InternalPath> {
        let decription = self.get_item_description(item_id)?;
    
        let mut file_name = String::new();
        if let Some(tag) = self.get_caption_tag(item_id)? {
            file_name = tag.to_string() + " - ";
        }
        file_name += &decription.name;

        let mut path = self.folders.gen_internal_path(decription.folder_id)?;
        path.push(&template.replace("{file_name}", &file_name));

        return Ok(path);
    }
}

impl ServiceApi for MusicCollection {

}

impl ServiceInitializer for MusicCollection {
    fn initialize(context: &AppContext) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();
        let database = context.get_service::<Database>();

        let music = Arc::new(Self {
            music_db: Arc::new(database.get_music_api()),
            tags_db: Arc::new(database.get_tags_api()),
            folders: context.get_service::<FoldersCollection>(),
        });

        register_rpc_handler!(rpc, music, "lappi.collection.music.get_tags", get_tags(item_id: MusicItemId));
        register_rpc_handler!(rpc, music, "lappi.collection.music.set_tag", set_tag(item_id: MusicItemId, tag_name: String, tag_value: TagValue));
        register_rpc_handler!(rpc, music, "lappi.collection.music.get_inheirted_tags", get_inherited_tags(item_id: MusicItemId));
        register_rpc_handler!(rpc, music, "lappi.collection.music.delete_tag", delete_tag(item_id: MusicItemId, tag_name: String));
        register_rpc_handler!(rpc, music, "lappi.collection.music.create_item", create_item(name: String, folder_id: FolderId));
        register_rpc_handler!(rpc, music, "lappi.collection.music.set_item_name", set_item_name(item_id: MusicItemId, name: String));
        register_rpc_handler!(rpc, music, "lappi.collection.music.get_item_description", get_item_description(item_id: MusicItemId));
        register_rpc_handler!(rpc, music, "lappi.collection.music.get_item_caption", get_item_caption(item_id: MusicItemId));

        return music;
    }
}
