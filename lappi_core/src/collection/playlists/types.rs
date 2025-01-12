use serde::{Serialize, Deserialize};

use crate::collection::music::MusicItemId;
use crate::collection::pictures::PictureId;

pub type PlaylistId = i64;
pub type PlaylistItemId = i64;

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct PlaylistDesc {
    pub id: PlaylistId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_picture_id: Option<PictureId>,
}

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct PlaylistItemDesc {
    pub id: PlaylistItemId,
    pub music_item_id: MusicItemId,
    pub title: String,
    pub album: String,
    pub artist: String,
}

