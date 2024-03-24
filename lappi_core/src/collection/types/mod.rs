pub mod tags;

pub type ItemId = i64;
pub type PictureId = i64;
pub type ArtistId = i64;
pub type EdgeId = i64;
pub type MusicItemId = ItemId;
pub type FolderId = i64;

#[derive(Copy, Clone, FromPrimitive)]
pub enum ItemType {
    Music = 0,
    Video = 1,
    Artist = 2,
    Album = 3,
}

pub struct ItemFullId {
    pub id: ItemId,
    pub item_type: ItemType,
}

