pub mod types;
pub mod database_api;

use std::path::PathBuf;
use std::sync::Arc;

use serde::{Serialize, Deserialize};
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Context, Service, ServiceApi, ServiceInitializer};

use crate::collection::storage::local::LocalStorage;
use crate::database::Database;

use super::folders::database_api::FoldersDbApi;
use super::music::database_api::MusicDbApi;
use super::music::MusicItemId;
use super::tags::database_api::TagsDbApi;
use super::tags::Tag;

pub use types::*;

pub struct FolderView {
    pub content_folders: Vec<(MusicItemId, String)>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemDescription {
    pub item_id: MusicItemId,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct FolderContent {
    pub folders: Vec<FolderDescription>,
    pub items: Vec<ItemDescription>,
}

#[derive(Serialize, Deserialize)]
pub struct FolderFullContent {
    content: FolderContent,
    folders_chain: Vec<FolderDescription>,
}

pub struct FoldersCollection {
    local_storage: Service<LocalStorage>,
    folders_db: Arc<Box<dyn FoldersDbApi>>,
    tags_db: Arc<Box<dyn TagsDbApi>>,
    music_db: Arc<Box<dyn MusicDbApi>>,
}

impl FoldersCollection {
    pub fn get_root_folder(&self) -> FolderId {
        self.folders_db.get_root_folder()
    }

    pub fn get_folder_parent(&self, folder_id: FolderId) -> FolderId {
        self.folders_db.get_folder_parent(folder_id).unwrap()
    }

    pub fn get_folder_name(&self, folder_id: FolderId) -> String {
        self.folders_db.get_folder_name(folder_id).unwrap()
    }

    pub fn get_folder_description(&self, folder_id: FolderId) -> FolderDescription {
        self.folders_db.get_folder_description(folder_id).unwrap()
    }

    pub fn set_folder_name(&self, folder_id: FolderId, name: String) {
        self.folders_db.set_folder_name(folder_id, &name).unwrap();
    }

    pub fn set_folder_type(&self, folder_id: FolderId, folder_type: FolderType) {
        self.folders_db.set_folder_type(folder_id, folder_type).unwrap();
    }

    pub fn find_or_add_folder(&self, parent_id: FolderId, folder_name: String, folder_type: FolderType) -> FolderId {
        self.folders_db.find_or_add_folder(parent_id, folder_name.as_str(), folder_type).unwrap()
    }

    pub fn get_folders_in_folder(&self, folder_id: FolderId) -> Vec<FolderDescription> {
        self.folders_db.get_folders_in_folder(folder_id).unwrap()
    }

    pub fn find_parent_node(&self, folder_id: FolderId, folder_type: FolderType) -> Option<FolderDescription> {
        let parent_folders = self.get_folders_chain(folder_id);
        return parent_folders.iter().find(|f| f.folder_type == folder_type).cloned();
    }

    pub fn get_folder_content(&self, folder_id: FolderId) -> FolderContent {
        let items_id = self.folders_db.get_music_items_in_folder(folder_id).unwrap();
        let mut items = Vec::new();
        for item_id in items_id {
            let name = self.music_db.get_music_item_description(item_id).unwrap().name;
            items.push(ItemDescription {
                item_id,
                name,
            })
        }

        let folders = self.get_folders_in_folder(folder_id);

        FolderContent {
            folders,
            items
        }
    }

    pub fn get_folders_chain(&self, folder_id: FolderId) -> Vec<FolderDescription> {
        if folder_id != self.folders_db.get_root_folder() {
            let mut chain = Vec::new();
            let mut next_folder_id = folder_id;
            loop {
                let folder_desc = self.folders_db.get_folder_description(next_folder_id).unwrap();
                let parent_id = self.folders_db.get_folder_parent(next_folder_id).unwrap();
                chain.push(folder_desc);
                if parent_id == self.folders_db.get_root_folder() {
                    chain.reverse();
                    return chain;
                } else {
                    next_folder_id = parent_id;
                }
            }
        } else {
            return vec![];
        }
    }

    pub fn get_folder_full_content(&self, folder_id: FolderId) -> FolderFullContent {
        let content = self.get_folder_content(folder_id);
        let folders_chain = self.get_folders_chain(folder_id);

        FolderFullContent {
            content,
            folders_chain
        }
    }

    pub fn set_tag(&self, folder_id: FolderId, tag_name: String, tag_value: String) {
        self.tags_db.set_add_folder_tag(folder_id, tag_name.as_str(), tag_value.as_str()).unwrap();
    }

    pub fn get_own_folder_tag(&self, folder_id: FolderId) -> Option<Tag> {
        let description = self.folders_db.get_folder_description(folder_id).unwrap();
        match description.folder_type {
            FolderType::Folder => {
                return None;
            }
            FolderType::Album => {
                return Some(Tag::new_string("album".to_string(), description.name));
            }
            FolderType::Artist => {
                return Some(Tag::new_string("artist".to_string(), description.name));
            }
        }
    }

    pub fn get_tags(&self, folder_id: FolderId) -> Vec<Tag> {
        self.tags_db.get_folder_tags(folder_id).unwrap()
    }

    pub fn get_inherited_tags(&self, folder_id: FolderId) -> Vec<Tag> {
        let mut tags = vec![];

        let own_tag = self.get_own_folder_tag(folder_id);
        if let Some(own_tag) = own_tag {
            tags.push(own_tag);
        }

        let parent_folder_id = self.folders_db.get_folder_parent(folder_id).unwrap();
        if self.get_root_folder() != parent_folder_id {
            let parent_tags = self.get_inherited_tags(parent_folder_id);
            tags.extend(parent_tags);
        }

        return tags;
    }

    pub fn save_description(&self, folder_id: FolderId, text: String) {
        let path = self.get_description_storage_path(folder_id);
        std::fs::write(path, text.as_bytes()).unwrap();
    }

    pub fn get_description(&self, folder_id: FolderId) -> String {
        let path = self.get_description_storage_path(folder_id);
        let file_content = std::fs::read_to_string(path).unwrap_or("".to_string());
        return file_content;
    }

    fn get_description_storage_path(&self, folder_id: FolderId) -> PathBuf {
        return self.local_storage.get_internal_storage_folder("folders/about").join(format!("{}.txt", folder_id));
    }
}

impl ServiceApi for FoldersCollection {

}

impl ServiceInitializer for FoldersCollection {
    fn initialize(context: &Context) -> Arc<Self> {
        let local_storage = context.get_service::<LocalStorage>();
        let rpc: Service<Rpc> = context.get_service::<Rpc>();
        let database = context.get_service::<Database>();

        let folders = Arc::new(Self {
            local_storage,
            folders_db: Arc::new(database.get_folders_api()),
            tags_db: Arc::new(database.get_tags_api()),
            music_db: Arc::new(database.get_music_api()),
        });

        register_rpc_handler!(rpc, folders, "lappi.collection.folders.get_folder_description", get_folder_description(folder_id: FolderId));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.get_folder_content", get_folder_full_content(folder_id: FolderId));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.get_parent_folders", get_folders_chain(folder_id: FolderId));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.set_folder_name", set_folder_name(folder_id: FolderId, name: String));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.set_folder_type", set_folder_type(folder_id: FolderId, folder_type: FolderType));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.find_or_add_folder", find_or_add_folder(parent_id: FolderId, folder_name: String, folder_type: FolderType));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.get_tags", get_tags(folder_id: FolderId));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.set_tag", set_tag(folder_id: FolderId, tag_name: String, tag_value: String));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.get_inheirted_tags", get_inherited_tags(folder_id: FolderId));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.save_description", save_description(folder_id: FolderId, text: String));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.get_description", get_description(folder_id: FolderId));

        return folders;
    }
}

