use std::fmt::Debug;

use crate::collection::music::{MusicCollection, MusicItemId};
use crate::collection::pictures::PictureId;

#[derive(Clone, Debug)]
pub enum SourceType {
    LocalFile(String),
}

#[derive(Clone, Debug)]
pub struct PlaybackSource {
    name: String,
    source_type: SourceType,
    cover_picture: Option<PictureId>,
}

impl PlaybackSource {
    pub fn local_file(name: String, path: String) -> Box<PlaybackSource> {
        Box::new(Self {
            name,
            source_type: SourceType::LocalFile(path),
            cover_picture: Option::None,
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
    
        let item_desc = music.get_item_description(music_item_id);
        let artist_tag = music.get_tag(music_item_id, "artist");
        let name = if let Some(artist_tag) = artist_tag {
            format!("{} - {}", artist_tag.get_string().unwrap(), item_desc.name)
        } else {
            item_desc.name
        };

        if let Some(file_desc) = default_source_file {
            let mut playback_source = Self::local_file(name, file_desc.path);
            playback_source.cover_picture = music.get_item_cover(music_item_id);
            return Some(playback_source);
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

    pub fn get_cover_picture(&self) -> Option<PictureId> {
        self.cover_picture
    }
}
