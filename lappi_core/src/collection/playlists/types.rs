use serde::{Serialize, Deserialize};

use crate::collection::music::MusicItemId;

pub type PlaylistId = i64;
pub type PlaylistItemId = i64;

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct PlaylistDesc {
    pub id: PlaylistId,
    pub name: String,
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

