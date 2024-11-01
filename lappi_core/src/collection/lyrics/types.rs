use serde::{Deserialize, Serialize};

pub type LyricsId = i64;

#[derive(Serialize, Deserialize)]
pub struct LyricsDescription {
    pub lyrics_id: LyricsId,
    pub lang_code: String,
}

