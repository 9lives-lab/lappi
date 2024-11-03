pub mod types;
pub mod database_api;

use std::sync::Arc;

use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::Context;

use crate::collection::folders::FolderId;
use crate::database::Database;

use database_api::MusicDbApi;
pub use types::*;

pub struct ItemRef {
    db: Arc<Box<dyn MusicDbApi>>,
    id: MusicItemId,
}

impl ItemRef {

    pub fn get_item_id(&self) -> MusicItemId {
        self.id
    }

    pub fn add_tag(&self, key: &str, value: &str) {
        self.db.add_tag(self.id, key, value).unwrap();
    }

    pub fn get_tag(&self, key: &str) -> Option<Tag> {
        self.db.get_tag(self.id, key).unwrap()
    }

    pub fn get_tags(&self) -> Vec<Tag> {
        self.db.get_tags(self.id).unwrap()
    }

}

pub struct ItemsIterator {
    db: Arc<Box<dyn MusicDbApi>>,
    ids: Vec<MusicItemId>,
    index: usize,
}

impl Iterator for ItemsIterator {
    type Item = ItemRef;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.ids.len() {
            let result = Some(ItemRef {
                db: self.db.clone(),
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
    db: Arc<Box<dyn MusicDbApi>>,
}

impl MusicCollection {

    pub fn initialize(context: &Context) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();
        let database = context.get_service::<Database>();
        let db_api = Arc::new(database.get_music_api());

        let music = Arc::new(Self {
            db: db_api,
        });

        register_rpc_handler!(rpc, music, "lappi.collection.get_tags", get_tags(item_id: MusicItemId));
        register_rpc_handler!(rpc, music, "lappi.collection.music.create_item", create_item(name: String, folder_id: FolderId));
        register_rpc_handler!(rpc, music, "lappi.collection.music.set_item_name", set_item_name(item_id: MusicItemId, name: String));
        register_rpc_handler!(rpc, music, "lappi.collection.music.get_item_description", get_item_description(item_id: MusicItemId));
        register_rpc_handler!(rpc, music, "lappi.collection.music.add_source_file", add_source_file(item_id: MusicItemId, source_type: SourceType, path: String));
        register_rpc_handler!(rpc, music, "lappi.collection.music.delete_source_file", delete_source_file(source_id: MusicSourceFileId));
        register_rpc_handler!(rpc, music, "lappi.collection.music.set_source_file_path", set_source_file_path(source_id: MusicSourceFileId, path: String));
        register_rpc_handler!(rpc, music, "lappi.collection.music.get_source_files", get_source_files(item_id: MusicItemId));

        return music;
    }

    pub fn batch(db: Arc<Box<dyn MusicDbApi>>) -> Arc<Self> {
        Arc::new(Self {
            db,
        })
    }

    pub fn create_item(&self, name: String, folder_id: FolderId) -> MusicItemId {
        return self.db.add_music_item(&name, folder_id)
    }

    pub fn set_item_name(&self, item_id: MusicItemId, name: String) {
        self.db.set_item_name(item_id, &name).unwrap();
    }

    pub fn get_item(&self, id: MusicItemId) -> ItemRef {
        ItemRef {
            db: self.db.clone(),
            id,
        }
    }

    pub fn get_item_description(&self, item_id: MusicItemId) -> MusicItemDescription {
        self.db.get_music_item_description(item_id).unwrap()
    }

    pub fn add_tag(&self, item_id: MusicItemId, key: &str, value: &str) {
        self.db.add_tag(item_id, key, value).unwrap();
    }

    pub fn get_tags(&self, item_id: MusicItemId) -> Vec<Tag> {
        self.get_item(item_id).get_tags()
    }

    pub fn add_source_file(&self, item_id: MusicItemId, source_type: SourceType, path: String) {
        self.db.add_source_file(item_id, source_type, &path).unwrap();
    }

    pub fn delete_source_file(&self, source_id: MusicSourceFileId) {
        self.db.delete_source_file(source_id).unwrap();
    }

    pub fn set_source_file_path(&self, source_id: MusicSourceFileId, path: String) {
        self.db.set_source_file_path(source_id, &path).unwrap();
    }

    pub fn get_source_files(&self, item_id: MusicItemId) -> Vec<SourceFileDesc> {
        self.db.get_source_files(item_id).unwrap()
    }

}

