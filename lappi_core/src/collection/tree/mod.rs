pub mod structure;

use std::ops::Deref;
use std::sync::Arc;

use serde::{Serialize, Deserialize};
use amina_core::events::{Event, EventEmitter};
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Context, Service};

use crate::collection::database_api::DatabaseApi;
use crate::collection::types::{ArtistId, FolderId, ItemId};
use crate::collection::tree::structure::{FolderContentDescriptor, FolderDescriptor};
use crate::database::Database;

pub struct FolderView {
    pub content_folders: Vec<(ItemId, String)>,
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum FolderType {
    Tag = 0,
    Artist = 1,
}

#[derive(Serialize, Deserialize)]
pub enum FolderDetails {
    None,
    Artist(ArtistId),
}

#[derive(Serialize, Deserialize)]
pub struct FolderDescription {
    pub folder_id: FolderId,
    pub title: String,
    pub folder_type: FolderType,
    pub details: FolderDetails,
}

#[derive(Serialize, Deserialize)]
pub struct ItemDescription {
    pub item_id: ItemId,
    pub title: String,
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
#[key = "lappi.collection.tree.OnFoldersUpdated"]
pub struct OnFoldersUpdated {
    pub tree_updated: bool,
}

pub struct CollectionView {
    event_emitter: Service<EventEmitter>,
    db: Arc<Box<dyn DatabaseApi>>,
    folder_descriptor: FolderDescriptor,
}

impl CollectionView {

    pub fn initialize(context: &Context) -> Arc<Self> {
        let event_emitter = context.get_service::<EventEmitter>();
        let rpc = context.get_service::<Rpc>();
        let database = context.get_service::<Database>();
        let db_api = Arc::new(database.collection());

        let tree = Arc::new(Self {
            event_emitter,
            db: db_api,
            folder_descriptor: FolderDescriptor::default(),
        });

        register_rpc_handler!(rpc, tree, "lappi.collection.tree.get_folder_description", get_folder_description(folder_id: FolderId));
        register_rpc_handler!(rpc, tree, "lappi.collection.view.get_folder_content", get_folder_full_content(folder_id: FolderId));
        register_rpc_handler!(rpc, tree, "lappi.collection.view.get_parent_folders", get_folders_chain(folder_id: FolderId));

        return tree;
    }

    pub fn update_item(&self, item_id: ItemId) {
        let mut current_folder_id = self.db.get_root_folder();
        let mut current_folder = &self.folder_descriptor;
        loop {
            match current_folder.content.deref() {
                // If current folder contains artists
                FolderContentDescriptor::ArtistsFolders(folder_content) => {
                    let artists_list = self.db.get_artists_by_collection_item(item_id).unwrap();
                    let artist_id = *artists_list.get(0).unwrap();
                    // get artist name
                    let artist_name = self.db.get_artist_name(artist_id).unwrap();
                    // Get next folder id or create this folder if it does not exist yet
                    let folder_id = self.db.find_or_add_folder(
                        current_folder_id, artist_name.as_str(), FolderType::Artist
                    ).unwrap();
                    // Set current folder for artist
                    self.db.set_folder_for_artist(artist_id, folder_id).unwrap();
                    // Update current folder id
                    current_folder_id = folder_id;
                    // Update current folder description ref
                    current_folder = &folder_content.folder_description;
                },
                // If current folder contains other folders
                FolderContentDescriptor::TagFolders(folder_content) => {
                    // Check which tag will be used as folder name
                    let current_tag = folder_content.tag.as_str();
                    // Get value of this tag in updated item
                    match self.db.get_tag(item_id, current_tag).unwrap() {
                        // If updated item has this tag
                        Some(tag) => {
                            let tag_value = tag.get_string().unwrap();
                            // Get next folder id or create this folder if it does not exist yet
                            let folder_id = self.db.find_or_add_folder(
                                current_folder_id, tag_value.as_str(), FolderType::Tag
                            ).unwrap();
                            // Update current folder id
                            current_folder_id = folder_id;
                            // Update current folder description ref
                            current_folder = &folder_content.folder_description;
                        },
                        // If updated item hasn't this tag
                        None => {
                            // Set this folder for item
                            self.db.set_folder_for_item(item_id, current_folder_id).unwrap();
                            // And exit loop
                            self.notify();
                            return;
                        }
                    }
                },
                // If current folder contains only items
                FolderContentDescriptor::Items => {
                    // Set this folder for item
                    self.db.set_folder_for_item(item_id, current_folder_id).unwrap();
                    // And exit loop
                    self.notify();
                    return;
                },
            }
        }
    }

    pub fn get_folder_description(&self, folder_id: FolderId) -> FolderDescription {
        self.db.get_folder_description(folder_id).unwrap()
    }

    pub fn get_folder_content(&self, folder_id: FolderId) -> FolderContent {
        let items_id = self.db.get_items_in_folder(folder_id).unwrap();
        let mut items = Vec::new();
        for item_id in items_id {
            let tag_option = self.db.get_tag(item_id, "title").unwrap();
            let tag = tag_option.unwrap();
            items.push(ItemDescription {
                item_id,
                title: tag.get_string().unwrap(),
            })
        }

        let folders = self.db.get_folders_in_folder(folder_id).unwrap();

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

    fn notify(&self) {
        self.event_emitter.emit_event(&OnFoldersUpdated {
            tree_updated: true,
        });
    }

}

