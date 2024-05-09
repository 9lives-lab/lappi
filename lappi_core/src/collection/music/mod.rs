pub mod types;

use std::sync::Arc;

use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::Context;

use crate::collection::database_api::DatabaseApi;
use crate::collection::music::types::ExternalSrcFileDesc;
use crate::collection::types::{FolderId, ItemId};
use crate::collection::types::tags::Tag;
use crate::database::Database;

pub struct ItemRef {
    db: Arc<Box<dyn DatabaseApi>>,
    id: ItemId,
}

impl ItemRef {

    pub fn get_item_id(&self) -> ItemId {
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
    db: Arc<Box<dyn DatabaseApi>>,
    ids: Vec<ItemId>,
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
    db: Arc<Box<dyn DatabaseApi>>,
}

impl MusicCollection {

    pub fn initialize(context: &Context) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();
        let database = context.get_service::<Database>();
        let db_api = Arc::new(database.collection());

        let music = Arc::new(Self {
            db: db_api,
        });

        register_rpc_handler!(rpc, music, "lappi.collection.get_tags", get_tags(item_id: ItemId));
        register_rpc_handler!(rpc, music, "lappi.collection.music.add_external_src_file", add_external_src_file(item_id: ItemId, path: String));
        register_rpc_handler!(rpc, music, "lappi.collection.music.get_external_src_files", get_external_src_files(item_id: ItemId));

        return music;
    }

    pub fn batch(db: Arc<Box<dyn DatabaseApi>>) -> Arc<Self> {
        Arc::new(Self {
            db,
        })
    }

    pub fn create_item(&self, name: &str, folder_id: FolderId) -> ItemId {
        return self.db.add_music_item(name, folder_id)
    }

    pub fn get_item(&self, id: ItemId) -> ItemRef {
        ItemRef {
            db: self.db.clone(),
            id,
        }
    }

    pub fn add_tag(&self, item_id: ItemId, key: &str, value: &str) {
        self.db.add_tag(item_id, key, value).unwrap();
    }

    pub fn get_tags(&self, item_id: ItemId) -> Vec<Tag> {
        self.get_item(item_id).get_tags()
    }

    pub fn add_external_src_file(&self, item_id: ItemId, path: String) {
        self.db.add_external_src_file(item_id, &path).unwrap();
    }

    pub fn get_external_src_files(&self, item_id: ItemId) -> Vec<ExternalSrcFileDesc> {
        self.db.get_external_src_files(item_id).unwrap()
    }

}

