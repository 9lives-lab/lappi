use anyhow::Result;

use crate::collection::internal_files::InternalFileId;
use crate::collection::music::MusicItemId;
use super::{LyricsDesc, LyricsId};

pub trait LyricsDbApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn LyricsDbApi>;
    fn add_lyrics_item(&self, music_id: MusicItemId, lyrics_tag: &str, internal_file_id: InternalFileId) -> Result<LyricsId>;
    fn get_lyrics_descriptor(&self, lyrics_id: LyricsId) -> Result<LyricsDesc>;
    fn get_lyrics_list(&self, music_id: MusicItemId) -> Result<Vec<LyricsDesc>>;
}
