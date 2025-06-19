use anyhow::Result;
use serde::Deserialize;
use reqwest::blocking::Client;

use crate::exploring::lyrics::sources::LyricsSourceApi;

#[derive(Deserialize, Debug)]
struct LyricsResponse {
    lyrics: String,
}

pub struct LyricsOvhSource {

}

impl LyricsOvhSource {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl LyricsSourceApi for LyricsOvhSource {
    fn source_name(&self) -> &str {
        return "lyrics.ovh";
    }

    fn find_lyrics(&self, artist: &str, title: &str) -> Result<String> {
        let url = format!("https://api.lyrics.ovh/v1/{}/{}", artist, title);

        let client = Client::new();
        let response = client.get(&url).send()?;
        let response = response.error_for_status()?;
        let lyrics_response: LyricsResponse = response.json()?;

        let lyrics = lyrics_response.lyrics;

        // clean string
        let lyrics = lyrics.replace("\\n", "\n"); // replace \n with newline
        let lyrics = lyrics.replace("\r", ""); // remove carriage returns
        let lyrics = lyrics.replace("\n\n", "\n"); // remove double newlines

        
        return Ok(lyrics);
    }
}

