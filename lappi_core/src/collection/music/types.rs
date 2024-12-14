use serde::{Serialize, Deserialize};
use num_derive::FromPrimitive;

use crate::collection::folders::FolderId;

pub type MusicItemId = i64;
pub type MusicSourceFileId = i64;

#[derive(Serialize, Deserialize)]
pub struct MusicItemDescription {
    pub item_id: MusicItemId,
    pub name: String,
    pub folder_id: FolderId,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, Serialize, Deserialize)]
pub enum SourceType {
    CollectionFile = 0,
    LocalFile = 1,
    Url = 2,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceFileDesc {
    pub id: MusicSourceFileId,
    pub music_item_id: MusicItemId,
    pub path: String,
    pub source_type: SourceType,
}
