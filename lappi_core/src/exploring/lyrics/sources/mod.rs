pub mod lyrics_ovh;

use std::ops::Deref;
use anyhow::Result;
use crate::exploring::ExploringSource;

pub trait LyricsSourceApi: Send + Sync {
    fn source_name(&self) -> &str;
    fn find_lyrics(&self, artist_name: &str, song_name: &str) -> Result<String>;
}

pub struct LyricsSource {
    api: Box<dyn LyricsSourceApi>,
}

impl LyricsSource {
    pub fn new<T: LyricsSourceApi + 'static>(api: T) -> Self {
        Self {
            api: Box::new(api)
        }
    }
}

impl ExploringSource for LyricsSource {
    fn source_name(&self) -> &str {
        self.api.source_name()
    }
}

impl Deref for LyricsSource {
    type Target = dyn LyricsSourceApi;
    fn deref(&self) -> &Self::Target {
        self.api.deref()
    }
}
