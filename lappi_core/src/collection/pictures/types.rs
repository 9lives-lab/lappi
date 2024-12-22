use serde::{Serialize, Deserialize};

pub type PictureId = i64;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct PictureBlob {
    pub file_name: String,
    pub file_type: String,
    pub data_base64: String,
}
