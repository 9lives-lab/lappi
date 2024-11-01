use serde::{Serialize, Deserialize};

pub type PlaylistId = i64;

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct PlaylistDesc {
    pub id: PlaylistId,
    pub name: String,
}
