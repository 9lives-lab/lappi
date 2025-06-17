use serde::{Deserialize, Serialize};

use crate::collection::internal_files::InternalFileId;

pub type LyricsId = i64;

#[derive(Serialize, Deserialize)]
pub struct LyricsDesc {
    pub lyrics_id: LyricsId,
    pub lyrics_tag: String,
    pub internal_file_id: InternalFileId,
}

