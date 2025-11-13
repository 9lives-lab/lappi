pub mod types;
pub mod database_api;

use std::sync::Arc;

use anyhow::Result;
use serde::{Serialize, Deserialize};
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{AppContext, Service, ServiceApi, ServiceInitializer};

use crate::collection::internal_files::{InternalFileId, InternalFiles, InternalPath};
use crate::database::Database;

use super::folders::database_api::FoldersDbApi;
use super::music::database_api::MusicDbApi;
use super::music::MusicItemId;
use super::pictures::PictureId;
use super::tags::database_api::TagsDbApi;
use super::tags::{Tag, TagValue};

pub use types::*;

#[derive(Serialize, Deserialize)]
pub struct ItemDescription {
    pub item_id: MusicItemId,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct FolderContent {
    pub folders: Vec<FolderDesc>,
    pub items: Vec<ItemDescription>,
}

#[derive(Serialize, Deserialize)]
pub struct FolderFullContent {
    content: FolderContent,
    folders_chain: Vec<FolderDesc>,
}

pub struct FoldersCollection {
    internal_files: Service<InternalFiles>,
    folders_db: Arc<Box<dyn FoldersDbApi>>,
    tags_db: Arc<Box<dyn TagsDbApi>>,
    music_db: Arc<Box<dyn MusicDbApi>>,
}

impl FoldersCollection {
    pub fn is_empty(&self) -> bool {
        self.folders_db.is_empty()
    }

    pub fn get_root_folder(&self) -> FolderId {
        self.folders_db.get_root_folder()
    }

    pub fn get_all_folders(&self) -> Result<Vec<FolderId>> {
        self.folders_db.get_all_folders()
    }

    pub fn get_folder_parent(&self, folder_id: FolderId) -> Result<FolderId> {
        self.folders_db.get_folder_parent(folder_id)
    }

    pub fn get_folder_name(&self, folder_id: FolderId) -> Result<String> {
        self.folders_db.get_folder_name(folder_id)
    }

    pub fn get_folder_description(&self, folder_id: FolderId) -> Result<FolderDesc> {
        self.folders_db.get_folder_description(folder_id)
    }

    pub fn get_caption_tag(&self, folder_id: FolderId) -> Result<Option<Tag>> {
        match self.get_folder_description(folder_id)?.folder_type {
            FolderType::Album => self.get_tag(folder_id, "year", false),
            _ => Ok(None)
        }
    }

    pub fn get_folder_caption(&self, folder_id: FolderId) -> Result<String> {
        if let Some(tag) = self.get_caption_tag(folder_id)? {
            Ok(format!("{} - {}", tag.get_key(), tag.to_string()))
        } else {
            Ok("".to_string())
        }
    }

    pub fn set_folder_name(&self, folder_id: FolderId, name: String) -> Result<()> {
        self.folders_db.set_folder_name(folder_id, &name)
    }

    pub fn set_folder_type(&self, folder_id: FolderId, folder_type: FolderType) -> Result<()> {
        self.folders_db.set_folder_type(folder_id, folder_type)
    }

    pub fn set_folder_cover(&self, folder_id: FolderId, picture_id: PictureId) -> Result<()> {
        self.folders_db.set_folder_cover(folder_id, picture_id)
    }

    pub fn find_folder_cover(&self, folder_id: FolderId) -> Result<Option<PictureId>> {
        let folder_chain = self.get_folders_chain(folder_id)?;
        for folder in folder_chain.iter().rev() {
            if folder.avatar_picture_id.is_some() {
                return Ok(folder.avatar_picture_id);
            }
        }
        Ok(None)
    }

    pub fn find_or_add_folder(&self, parent_id: FolderId, folder_name: String, folder_type: FolderType) -> Result<FolderId> {
        self.folders_db.find_or_add_folder(parent_id, folder_name.as_str(), folder_type)
    }

    pub fn get_folders_in_folder(&self, folder_id: FolderId) -> Result<Vec<FolderDesc>> {
        self.folders_db.get_folders_in_folder(folder_id)
    }

    pub fn find_parent_node(&self, folder_id: FolderId, folder_type: FolderType) -> Result<Option<FolderDesc>> {
        let parent_folders = self.get_folders_chain(folder_id)?;
        Ok(parent_folders.iter().find(|f| f.folder_type == folder_type).cloned())
    }

    pub fn get_folder_content(&self, folder_id: FolderId) -> Result<FolderContent> {
        let items_id = self.folders_db.get_music_items_in_folder(folder_id)?;
        let mut items = Vec::new();
        for item_id in items_id {
            let name = self.music_db.get_music_item_description(item_id)?.name;
            items.push(ItemDescription {
                item_id,
                name,
            })
        }

        let folders = self.get_folders_in_folder(folder_id)?;

        Ok(FolderContent {
            folders,
            items
        })
    }

    pub fn get_folders_chain(&self, folder_id: FolderId) -> Result<Vec<FolderDesc>> {
        if folder_id != self.folders_db.get_root_folder() {
            let mut chain = Vec::new();
            let mut next_folder_id = folder_id;
            loop {
                let folder_desc = self.folders_db.get_folder_description(next_folder_id)?;
                let parent_id = self.folders_db.get_folder_parent(next_folder_id)?;
                chain.push(folder_desc);
                if parent_id == self.folders_db.get_root_folder() {
                    chain.reverse();
                    return Ok(chain);
                } else {
                    next_folder_id = parent_id;
                }
            }
        } else {
            return Ok(vec![]);
        }
    }

    pub fn get_folder_full_content(&self, folder_id: FolderId) -> Result<FolderFullContent> {
        let content = self.get_folder_content(folder_id)?;
        let folders_chain = self.get_folders_chain(folder_id)?;

        Ok(FolderFullContent {
            content,
            folders_chain
        })
    }

    pub fn set_tag(&self, folder_id: FolderId, tag_name: String, tag_value: TagValue) -> Result<()> {
        log::debug!("set_tag: folder_id: {}, tag_name: {}, tag_value: {:?}", folder_id, tag_name, tag_value);
        self.tags_db.set_add_folder_tag(folder_id, tag_name.as_str(), &tag_value)?;
        Ok(())
    }

    pub fn get_own_folder_tag(&self, folder_id: FolderId) -> Result<Option<Tag>> {
        let description = self.folders_db.get_folder_description(folder_id)?;
        Ok(match description.folder_type {
            FolderType::Folder => {
                None
            }
            FolderType::Album => {
                Some(Tag::new_string("album".to_string(), description.name))
            }
            FolderType::Artist => {
                Some(Tag::new_string("artist".to_string(), description.name))
            }
        })
    }

    pub fn get_tags(&self, folder_id: FolderId) -> Result<Vec<Tag>> {
        self.tags_db.get_folder_tags(folder_id)
    }

    pub fn get_inherited_tags(&self, folder_id: FolderId) -> Result<Vec<Tag>> {
        let mut tags = vec![];

        let own_tag = self.get_own_folder_tag(folder_id)?;
        if let Some(own_tag) = own_tag {
            tags.push(own_tag);
        }

        let parent_folder_id = self.folders_db.get_folder_parent(folder_id)?;
        if self.get_root_folder() != parent_folder_id {
            let parent_tags = self.get_tags(parent_folder_id)?;
            tags.extend(parent_tags);
            let parent_tags = self.get_inherited_tags(parent_folder_id)?;
            tags.extend(parent_tags);
        }

        Ok(tags)
    }

    pub fn get_tag(&self, folder_id: FolderId, tag_name: &str, include_inherited: bool) -> Result<Option<Tag>> {
        let mut tags = vec![];

        tags.extend(self.get_tags(folder_id)?);

        if include_inherited {
            tags.extend(self.get_inherited_tags(folder_id)?);
        }

        for tag in tags {
            if tag.get_key() == tag_name {
                return Ok(Some(tag));
            }
        }

        Ok(None)
    }

    pub fn delete_tag(&self, folder_id: FolderId, tag_name: String) -> Result<()> {
        self.tags_db.delete_folder_tag(folder_id, &tag_name)
    }

    pub fn save_description(&self, folder_id: FolderId, text: String) -> Result<()> {
        match self.folders_db.get_description_file(folder_id)? {
            Some(file_id) => {
                let path = self.internal_files.get_system_path(file_id)?;
                std::fs::write(path, text)?;
            },
            None => {
                let internal_path = self.gen_description_internal_path(folder_id)?;
                let file_id = self.internal_files.add_and_write_file(text.as_bytes(), &internal_path)?;
                self.folders_db.set_description_file(folder_id, file_id)?;
            }
        }
        Ok(())
    }

    pub fn get_description_file(&self, folder_id: FolderId) -> Result<Option<InternalFileId>> {
        self.folders_db.get_description_file(folder_id)
    }

    pub fn get_description(&self, folder_id: FolderId) -> Result<String> {
        match self.folders_db.get_description_file(folder_id)? {
            Some(file_id) => {
                let path = self.internal_files.get_system_path(file_id)?;
                let file = std::fs::read(path)?;
                Ok(String::from_utf8(file)?)
            }
            None => {
                Ok(String::new())
            }
        }
    }

    pub fn gen_internal_path(&self, folder_id: FolderId) -> Result<InternalPath> {
        let folders_chain = self.get_folders_chain(folder_id)?;
        let mut path = InternalPath::new();
        for folder_desc in folders_chain {
            let mut folder_name = String::new();
            if let Some(tag) = self.get_caption_tag(folder_desc.folder_id)? {
                folder_name = tag.to_string() + " - ";
            }
            folder_name += &folder_desc.name;
            
            path.push(&folder_name);
        }
        Ok(path)
    }

    pub fn gen_description_internal_path(&self, folder_id: FolderId) -> Result<InternalPath> {
        let mut path = self.gen_internal_path(folder_id)?;
        let file_name = self.get_folder_name(folder_id)? + ".txt";
        path.push(&file_name);
        Ok(path)
    }
}

impl ServiceApi for FoldersCollection {

}

impl ServiceInitializer for FoldersCollection {
    fn initialize(context: &AppContext) -> Arc<Self> {
        let internal_files = context.get_service::<InternalFiles>();
        let rpc: Service<Rpc> = context.get_service::<Rpc>();
        let database = context.get_service::<Database>();

        let folders = Arc::new(Self {
            internal_files,
            folders_db: Arc::new(database.get_folders_api()),
            tags_db: Arc::new(database.get_tags_api()),
            music_db: Arc::new(database.get_music_api()),
        });

        register_rpc_handler!(rpc, folders, "lappi.collection.folders.get_folder_description", get_folder_description(folder_id: FolderId));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.get_folder_caption", get_folder_caption(folder_id: FolderId));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.get_folder_content", get_folder_full_content(folder_id: FolderId));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.get_parent_folders", get_folders_chain(folder_id: FolderId));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.set_folder_name", set_folder_name(folder_id: FolderId, name: String));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.set_folder_type", set_folder_type(folder_id: FolderId, folder_type: FolderType));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.set_folder_cover", set_folder_cover(folder_id: FolderId, picture_id: PictureId));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.find_or_add_folder", find_or_add_folder(parent_id: FolderId, folder_name: String, folder_type: FolderType));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.get_tags", get_tags(folder_id: FolderId));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.set_tag", set_tag(folder_id: FolderId, tag_name: String, tag_value: TagValue));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.get_inheirted_tags", get_inherited_tags(folder_id: FolderId));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.delete_tag", delete_tag(folder_id: FolderId, tag_name: String));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.save_description", save_description(folder_id: FolderId, text: String));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.get_description", get_description(folder_id: FolderId));
        register_rpc_handler!(rpc, folders, "lappi.collection.folders.gen_internal_path", gen_internal_path(folder_id: FolderId));

        return folders;
    }
}

