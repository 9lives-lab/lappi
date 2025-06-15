use anyhow::Result;
use serde::{Serialize, Deserialize};

use crate::collection::{folders::FolderId, internal_files::InternalFileId};

pub type PictureId = i64;

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, Serialize, Deserialize)]
pub enum PictureType {
    JPG = 0,
    PNG = 1,
    GIF = 2,
    WEBP = 3,
    BMP = 4,
    AVIF = 5,
}

impl PictureType {
    pub fn from_str(file_extension: &str) -> Result<PictureType> {
        match file_extension {
            "jpg" => Ok(PictureType::JPG),
            "jpeg" => Ok(PictureType::JPG),
            "png" => Ok(PictureType::PNG),
            "gif" => Ok(PictureType::GIF),
            "webp" => Ok(PictureType::WEBP),
            "bmp" => Ok(PictureType::BMP),
            "avif" => Ok(PictureType::AVIF),
            _ => anyhow::bail!("Unknown file extension"),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            PictureType::JPG => "jpg",
            PictureType::PNG => "png",
            PictureType::GIF => "gif",
            PictureType::WEBP => "webp",
            PictureType::BMP => "bmp",
            PictureType::AVIF => "avif",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PictureDesc {
    pub picture_id: PictureId,
    pub folder_id: FolderId,
    pub internal_file_id: InternalFileId,
    pub picture_type: PictureType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PictureBlob {
    pub file_name: String,
    pub file_type: String,
    pub data_base64: String,
}
