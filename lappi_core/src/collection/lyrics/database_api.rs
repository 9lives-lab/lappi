use anyhow::Result;

use crate::collection::music::MusicItemId;
use super::{LyricsDescription, LyricsId};

pub trait LyricsDbApi: Send + Sync {
    fn clone_api(&self) -> Box<dyn LyricsDbApi>;
    fn add_lyrics_item(&self, music_id: MusicItemId, lang_code: &str) -> Result<LyricsId>;
    fn get_lyrics_list(&self, music_id: MusicItemId) -> Result<Vec<LyricsDescription>>;
}
