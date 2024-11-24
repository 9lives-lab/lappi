use std::fmt::Debug;

use crate::collection::music::{MusicCollection, MusicItemId};

#[derive(Clone, Debug)]
pub enum SourceType {
    LocalFile(String),
}

#[derive(Clone, Debug)]
pub struct PlaybackSource {
    pub name: String,
    pub source_type: SourceType,
}

impl PlaybackSource {
    pub fn local_file(name: String, path: String) -> Box<PlaybackSource> {
        Box::new(Self {
            name,
            source_type: SourceType::LocalFile(path),
        })
    }

    pub fn default_from_music_item(music_item_id: MusicItemId) -> Option<Box<PlaybackSource>> {
        let music = crate::context().get_service::<MusicCollection>();

        let mut default_source_file = None;
        let source_files = music.get_source_files(music_item_id);
    
        for file_desc in source_files {
            if file_desc.source_type == crate::collection::music::SourceType::LocalFile {
                default_source_file = Some(file_desc);
                break;
            }   
        }
    
        let name = music.get_item_description(music_item_id).name;

        if let Some(file_desc) = default_source_file {
            return Some(Self::local_file(name, file_desc.path));
        } else {
            return None;
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_source_type(&self) -> &SourceType {
        &self.source_type
    }
}
