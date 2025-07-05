pub mod mp3;

use std::collections::HashMap;
use std::io::Read;
use std::fs::File;

use anyhow::{Context, Result};
use camino::Utf8Path;
use serde::Serialize;

use crate::collection::tags::TagsMap;

#[derive(Debug, Clone, Serialize)]
pub struct Metadata {
    pub media_type: String,
    pub tags: TagsMap,
}

pub fn read(reader: Box<dyn Read>, extension: &str) -> Result<Option<Metadata>> {
    let mut metadata_readers: HashMap<String, Box<dyn Fn(Box<dyn Read>) -> Result<Metadata>>> = HashMap::new();
    metadata_readers.insert("mp3".to_string(), Box::new(mp3::read));

    Ok(if let Some(metadata_reader) = metadata_readers.get(extension) {
        Some(metadata_reader(reader)?)
    } else {
        None
    })
}

pub fn read_from_path(path: &Utf8Path) -> Result<Option<Metadata>> {
    let extension = path
        .extension()
        .context("File has no extension")?
        .to_string();
    let reader = File::open(path)?;
    read(Box::new(reader), extension.as_str())
}
