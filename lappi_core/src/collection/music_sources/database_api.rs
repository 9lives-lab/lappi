use anyhow::Result;

use crate::collection::music_sources::MusicFileDesc;
use crate::collection::music::MusicItemId;

use super::{MusicLinkId, MusicLinkDesc, MusicLinkType};

pub trait MusicSourcesDbApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn MusicSourcesDbApi>;

    // Music files
    fn add_music_file(&self, descriptor: &MusicFileDesc) -> Result<()>;
    fn get_music_file(&self, item_id: MusicItemId) -> Result<Option<MusicFileDesc>>;
    fn delete_music_file(&self, item_id: MusicItemId) -> Result<()>;

    // Music links
    fn add_music_link(&self, item_id: MusicItemId, link_type: MusicLinkType, link: &str) -> Result<()>;
    fn set_music_link(&self, link_id: MusicLinkId, link: &str) -> Result<()>;
    fn get_music_links(&self, item_id: MusicItemId) -> Result<Vec<MusicLinkDesc>>;
    fn delete_music_link(&self, link_id: MusicLinkId) -> Result<()>;

}
