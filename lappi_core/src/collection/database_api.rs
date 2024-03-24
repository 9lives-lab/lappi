use serde::{Serialize, Deserialize};
use amina_core::events::Event;

use crate::database_api::{DbExporter, DbImporter, DbResult};
use crate::collection::types::{ArtistId, EdgeId, FolderId, ItemId, ItemType, PictureId};
use crate::collection::types::tags::Tag;
use crate::collection::music::types::ExternalSrcFileDesc;
use crate::collection::tree::{FolderDescription, FolderType};

#[derive(Default, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Event)]
#[key = "lappi.collection.database.OnItemsUpdated"]
pub struct OnItemsUpdated {
    pub items: Vec<ItemId>,
    pub tags_updated: bool,
}

pub trait DatabaseApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn DatabaseApi>;
    fn start_batch(&self);
    fn stop_batch(&self);

    fn add_collection_item(&self) -> ItemId;
    fn get_collection_items(&self) -> DbResult<Vec<ItemId>>;

    // Pictures
    fn add_picture(&self, extension: &str) -> PictureId;
    fn get_picture_extension(&self, picture_id: PictureId) -> DbResult<String>;
    fn add_picture_to_artist(&self, picture_id: PictureId, artist_id: ArtistId) -> DbResult<()>;
    fn get_pictures_by_artist(&self, artist_id: ArtistId) -> DbResult<Vec<PictureId>>;

    // Artists
    fn find_or_add_artist(&self, name: &str) -> DbResult<ArtistId>;
    fn get_artist_name(&self, artist_id: ArtistId) -> DbResult<String>;
    fn add_artist_to_collection_item(&self, artist_id: ArtistId, item_id: ItemId) -> DbResult<()>;
    fn get_artists_by_collection_item(&self, item_id: ItemId) -> DbResult<Vec<ArtistId>>;

    // Graph
    fn add_node(&self, item_type: ItemType, name: &str) -> ItemId;
    fn find_or_add_node(&self, item_type: ItemType, name: &str) -> ItemId;
    fn get_node_type(&self, item_id: ItemId) -> ItemType;
    fn get_node_name(&self, item_id: ItemId) -> String;
    fn get_all_nodes(&self) -> Vec<ItemId>;

    fn find_or_add_edge(&self, first_node: ItemId, second_node: ItemId) -> EdgeId;
    fn get_edge(&self, edge_id: EdgeId) -> (ItemId, ItemId);
    fn get_all_edges(&self) -> Vec<EdgeId>;

    // Tags
    fn add_tag(&self, item_id: ItemId, name: &str, value: &str) -> DbResult<()>;
    fn get_tag(&self, item_id: ItemId, key: &str) -> DbResult<Option<Tag>>;
    fn get_tags(&self, item_id: ItemId) -> DbResult<Vec<Tag>>;

    // Song files
    fn add_external_src_file(&self, item_id: ItemId, path: &str) -> DbResult<()>;
    fn get_external_src_files(&self, item_id: ItemId) -> DbResult<Vec<ExternalSrcFileDesc>>;

    // Folders
    fn get_root_folder(&self) -> FolderId;
    fn get_item_folder(&self, item_id: ItemId) -> DbResult<FolderId>;
    fn get_folder_parent(&self, folder_id: FolderId) -> DbResult<FolderId>;
    fn get_folder_description(&self, folder_id: FolderId) -> DbResult<FolderDescription>;
    fn find_or_add_folder(&self, parent_id: FolderId, folder_name: &str, folder_type: FolderType) -> DbResult<FolderId>;
    fn get_folders_in_folder(&self, folder_id: FolderId) -> DbResult<Vec<FolderDescription>>;

    fn set_folder_for_artist(&self, artist_id: ArtistId, folder_id: FolderId) -> DbResult<()>;

    fn set_folder_for_item(&self, item_id: ItemId, folder_id: FolderId) -> DbResult<()>;
    fn get_items_in_folder(&self, folder_id: FolderId) -> DbResult<Vec<ItemId>>;

    fn export(&self, exporter: Box<dyn DbExporter>) -> DbResult<()>;
    fn import(&self, importer: Box<dyn DbImporter>) -> DbResult<()>;
}
