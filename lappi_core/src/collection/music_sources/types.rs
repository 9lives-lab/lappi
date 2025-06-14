use serde::{Serialize, Deserialize};

use crate::collection::{internal_files::InternalFileId, music::MusicItemId};

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, Serialize, Deserialize)]
pub enum MusicFileType {
    MP3 = 0,
    FLAC = 1,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MusicFileDesc {
    pub music_item_id: MusicItemId,
    pub internal_file_id: InternalFileId,
    pub file_type: MusicFileType,
}

pub type MusicLinkId = i64;

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, Serialize, Deserialize)]
pub enum MusicLinkType {
    ExternalFile = 0,
    Url = 1,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MusicLinkDesc {
    pub id: MusicLinkId,
    pub music_item_id: MusicItemId,
    pub link: String,
    pub link_type: MusicLinkType,
}

