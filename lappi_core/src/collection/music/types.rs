use serde::{Serialize, Deserialize};

use crate::collection::folders::FolderId;

pub type MusicItemId = i64;

#[derive(Serialize, Deserialize)]
pub struct MusicItemDescription {
    pub item_id: MusicItemId,
    pub name: String,
    pub folder_id: FolderId,
}

