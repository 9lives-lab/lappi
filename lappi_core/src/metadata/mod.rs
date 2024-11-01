pub mod mp3;

use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use std::fs::File;

use serde::Serialize;

use crate::collection::music::TagsMap;

#[derive(Debug, Clone, Serialize)]
pub struct Metadata {
    pub media_type: String,
    pub tags: TagsMap,
}

pub fn read(reader: Box<dyn Read>, extension: &str) -> Option<Metadata> {
    let mut metadata_readers: HashMap<String, Box<dyn Fn(Box<dyn Read>) -> Metadata>> = HashMap::new();
    metadata_readers.insert("mp3".to_string(), Box::new(mp3::read));
    let reader_option = metadata_readers.get(extension);
    reader_option.map(|metadata_reader| metadata_reader(reader))
}

pub fn read_from_path<P: AsRef<Path>>(path: P) -> Option<Metadata> {
    let extension = path.as_ref().extension().unwrap().to_str().unwrap().to_string();
    let reader = File::open(path).unwrap();
    return read(Box::new(reader), extension.as_str());
}
