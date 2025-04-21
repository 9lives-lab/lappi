pub mod types;
pub mod database_api;

use std::sync::Arc;

use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Context, Service, ServiceApi, ServiceInitializer};

use crate::database::Database;

use super::folders::{FolderId, FoldersCollection};
use super::pictures::PictureId;
use super::tags::database_api::TagsDbApi;
use super::tags::{Tag, TagValue};

use database_api::MusicDbApi;
pub use types::*;

pub struct ItemRef {
    _music_db: Arc<Box<dyn MusicDbApi>>,
    tags_db: Arc<Box<dyn TagsDbApi>>,
    id: MusicItemId,
}

impl ItemRef {

    pub fn get_item_id(&self) -> MusicItemId {
        self.id
    }

    pub fn add_tag(&self, tag_name: &str, value: &TagValue) {
        self.tags_db.set_add_item_tag(self.id, tag_name, value).unwrap();
    }

    pub fn get_tag(&self, tag_name: &str) -> Option<Tag> {
        self.tags_db.get_item_tag(self.id, tag_name).unwrap()
    }

    pub fn get_tags(&self) -> Vec<Tag> {
        self.tags_db.get_item_tags(self.id).unwrap()
    }

}

pub struct ItemsIterator {
    music_db: Arc<Box<dyn MusicDbApi>>,
    tags_db: Arc<Box<dyn TagsDbApi>>,
    ids: Vec<MusicItemId>,
    index: usize,
}

impl Iterator for ItemsIterator {
    type Item = ItemRef;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.ids.len() {
            let result = Some(ItemRef {
                _music_db: self.music_db.clone(),
                tags_db: self.tags_db.clone(),
                id: *self.ids.get(self.index).unwrap(),
            });
            self.index += 1;
            return result;
        } else {
            None
        }
    }
}

pub struct MusicCollection {
    music_db: Arc<Box<dyn MusicDbApi>>,
    tags_db: Arc<Box<dyn TagsDbApi>>,
    folders: Service<FoldersCollection>,
}

impl MusicCollection {
    pub fn create_item(&self, name: String, folder_id: FolderId) -> MusicItemId {
        return self.music_db.add_music_item(&name, folder_id)
    }

    pub fn set_item_name(&self, item_id: MusicItemId, name: String) {
        self.music_db.set_item_name(item_id, &name).unwrap();
    }

    pub fn get_item(&self, id: MusicItemId) -> ItemRef {
        ItemRef {
            _music_db: self.music_db.clone(),
            tags_db: self.tags_db.clone(),
            id,
        }
    }

    pub fn get_item_description(&self, item_id: MusicItemId) -> MusicItemDescription {
        self.music_db.get_music_item_description(item_id).unwrap()
    }

    pub fn get_item_caption(&self, item_id: MusicItemId) -> String {
        if let Some(tag) = self.get_tag(item_id, "track") {
            if let TagValue::Number(track) = tag.get_value() {
                return format!("track - {}", track);
            }
        }
        return "".to_string();
    }

    pub fn get_item_cover(&self, item_id: MusicItemId) -> Option<PictureId> {
        self.folders.find_folder_cover(self.get_item_description(item_id).folder_id)
    }

    pub fn set_tag(&self, item_id: MusicItemId, tag_name: String, tag_value: TagValue) {
        self.tags_db.set_add_item_tag(item_id, tag_name.as_str(), &tag_value).unwrap();
    }

    pub fn get_tags(&self, item_id: MusicItemId) -> Vec<Tag> {
        self.tags_db.get_item_tags(item_id).unwrap()
    }

    pub fn get_inherited_tags(&self, item_id: MusicItemId) -> Vec<Tag> {
        let mut tags = vec![];

        let decription = self.get_item_description(item_id);
        let folder_id = decription.folder_id;

        tags.extend(self.folders.get_tags(folder_id));
        tags.extend(self.folders.get_inherited_tags(folder_id));

        return tags;
    }

    pub fn get_tag(&self, item_id: MusicItemId, tag_name: &str) -> Option<Tag> {
        let mut tags = vec![];

        tags.extend(self.get_tags(item_id));
        tags.extend(self.get_inherited_tags(item_id));

        for tag in tags {
            if tag.get_key() == tag_name {
                return Some(tag);
            }
        }

        return None;
    }

    pub fn delete_tag(&self, item_id: MusicItemId, tag_name: String) {
        self.tags_db.delete_item_tag(item_id, &tag_name).unwrap();
    }

    pub fn add_source_file(&self, item_id: MusicItemId, source_type: SourceType, path: String) {
        self.music_db.add_source_file(item_id, source_type, &path).unwrap();
    }

    pub fn delete_source_file(&self, source_id: MusicSourceFileId) {
        self.music_db.delete_source_file(source_id).unwrap();
    }

    pub fn set_source_file_path(&self, source_id: MusicSourceFileId, path: String) {
        self.music_db.set_source_file_path(source_id, &path).unwrap();
    }

    pub fn get_source_files(&self, item_id: MusicItemId) -> Vec<SourceFileDesc> {
        self.music_db.get_source_files(item_id).unwrap()
    }

}

impl ServiceApi for MusicCollection {

}

impl ServiceInitializer for MusicCollection {
    fn initialize(context: &Context) -> Arc<Self> {
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
        register_rpc_handler!(rpc, music, "lappi.collection.music.add_source_file", add_source_file(item_id: MusicItemId, source_type: SourceType, path: String));
        register_rpc_handler!(rpc, music, "lappi.collection.music.delete_source_file", delete_source_file(source_id: MusicSourceFileId));
        register_rpc_handler!(rpc, music, "lappi.collection.music.set_source_file_path", set_source_file_path(source_id: MusicSourceFileId, path: String));
        register_rpc_handler!(rpc, music, "lappi.collection.music.get_source_files", get_source_files(item_id: MusicItemId));

        return music;
    }
}
