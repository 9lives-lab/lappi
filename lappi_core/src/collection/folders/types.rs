use serde::{Serialize, Deserialize};

use crate::collection::pictures::PictureId;

pub type FolderId = i64;
pub type ArtistId = FolderId;
pub type AlbumId = FolderId;

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

