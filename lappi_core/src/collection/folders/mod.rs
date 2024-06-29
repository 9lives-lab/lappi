use std::sync::Arc;

use serde::{Serialize, Deserialize};
use amina_core::events::{Event, EventEmitter};
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Context, Service};

use crate::collection::database_api::DatabaseApi;
use crate::collection::types::{FolderId, ItemId, PictureId};
use crate::database::Database;

pub struct FolderView {
    pub content_folders: Vec<(ItemId, String)>,
}

#[derive(Copy, Clone, PartialEq, Eq, FromPrimitive, Serialize, Deserialize)]
pub enum FolderType {
    Folder = 0,
    Artist = 1,
    Album = 2,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FolderDescription {
    pub folder_id: FolderId,
    pub name: String,
    pub folder_type: FolderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_picture_id: Option<PictureId>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemDescription {
    pub item_id: ItemId,
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

#[derive(Default, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Event)]
#[key = "lappi.collection.folders.OnFoldersUpdated"]
pub struct OnFoldersUpdated {
    pub tree_updated: bool,
}

pub struct FoldersView {
    event_emitter: Service<EventEmitter>,
    db: Arc<Box<dyn DatabaseApi>>,
}

impl FoldersView {
    pub fn initialize(context: &Context) -> Arc<Self> {
        let event_emitter = context.get_service::<EventEmitter>();
        let rpc = context.get_service::<Rpc>();
        let database = context.get_service::<Database>();
        let db_api = Arc::new(database.collection());

        let folders = Arc::new(Self {
            event_emitter,
            db: db_api,
        });

        register_rpc_handler!(rpc, folders, "lappi.collection.folders.get_folder_description", get_folder_description(folder_id: FolderId));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.get_folder_content", get_folder_full_content(folder_id: FolderId));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.get_parent_folders", get_folders_chain(folder_id: FolderId));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.find_or_add_folder", find_or_add_folder(parent_id: FolderId, folder_name: String, folder_type: FolderType));

        return folders;
    }

    pub fn get_root_folder(&self) -> FolderId {
        self.db.get_root_folder()
    }

    pub fn get_folder_parent(&self, folder_id: FolderId) -> FolderId {
        self.db.get_folder_parent(folder_id).unwrap()
    }

    pub fn get_folder_name(&self, folder_id: FolderId) -> String {
        self.db.get_folder_name(folder_id).unwrap()
    }

    pub fn get_folder_description(&self, folder_id: FolderId) -> FolderDescription {
        self.db.get_folder_description(folder_id).unwrap()
    }

    pub fn find_or_add_folder(&self, parent_id: FolderId, folder_name: String, folder_type: FolderType) -> FolderId {
        self.db.find_or_add_folder(parent_id, folder_name.as_str(), folder_type).unwrap()
    }

    pub fn get_folders_in_folder(&self, folder_id: FolderId) -> Vec<FolderDescription> {
        self.db.get_folders_in_folder(folder_id).unwrap()
    }

    pub fn find_parent_node(&self, folder_id: FolderId, folder_type: FolderType) -> Option<FolderDescription> {
        let parent_folders = self.get_folders_chain(folder_id);
        return parent_folders.iter().find(|f| f.folder_type == folder_type).cloned();
    }

    pub fn get_folder_content(&self, folder_id: FolderId) -> FolderContent {
        let items_id = self.db.get_music_items_in_folder(folder_id).unwrap();
        let mut items = Vec::new();
        for item_id in items_id {
            let tag_option = self.db.get_tag(item_id, "title").unwrap();
            let tag = tag_option.unwrap();
            items.push(ItemDescription {
                item_id,
                name: tag.get_string().unwrap(),
            })
        }

        let folders = self.get_folders_in_folder(folder_id);

        FolderContent {
            folders,
            items
        }
    }

    pub fn get_folders_chain(&self, folder_id: FolderId) -> Vec<FolderDescription> {
        if folder_id != self.db.get_root_folder() {
            let mut chain = Vec::new();
            let mut next_folder_id = folder_id;
            loop {
                let folder_desc = self.db.get_folder_description(next_folder_id).unwrap();
                let parent_id = self.db.get_folder_parent(next_folder_id).unwrap();
                chain.push(folder_desc);
                if parent_id == self.db.get_root_folder() {
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

    pub fn update_item(&self) {
        self.event_emitter.emit_event(&OnFoldersUpdated {
            tree_updated: true,
        });
    }
}

