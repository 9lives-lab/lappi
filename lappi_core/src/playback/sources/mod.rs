use std::fmt::Debug;
use std::path::PathBuf;

use anyhow::Result;

use crate::collection::internal_files::InternalFiles;
use crate::collection::music::{MusicCollection, MusicItemId};
use crate::collection::music_sources::MusicSourcesCollection;
use crate::collection::pictures::PictureId;

#[derive(Clone, Debug)]
pub enum SourceType {
    LocalFile(PathBuf),
}

#[derive(Clone, Debug)]
pub struct PlaybackSource {
    name: String,
    source_type: SourceType,
    cover_picture: Option<PictureId>,
}

impl PlaybackSource {
    pub fn local_file(name: String, path: PathBuf) -> Box<PlaybackSource> {
        Box::new(Self {
            name,
            source_type: SourceType::LocalFile(path),
            cover_picture: Option::None,
        })
    }

    pub fn default_from_music_item(music_item_id: MusicItemId) -> Result<Option<Box<PlaybackSource>>> {
        let music = crate::context().get_service::<MusicCollection>();
        let music_sources = crate::context().get_service::<MusicSourcesCollection>();
        let internal_files = crate::context().get_service::<InternalFiles>();

        match music_sources.get_music_file(music_item_id)? {
            Some(file_desc) => {
                let item_desc = music.get_item_description(music_item_id)?;

                let artist_tag = music.get_tag(music_item_id, "artist")?;
                let name = if let Some(artist_tag) = artist_tag {
                    format!("{} - {}", artist_tag.to_string(), item_desc.name)
                } else {
                    item_desc.name
                };

                let path = internal_files.get_system_path(file_desc.internal_file_id)?;

                let mut playback_source = Self::local_file(name, path);
                playback_source.cover_picture = music.get_item_cover(music_item_id)?;

                Ok(Some(playback_source))
            },
            None => Ok(None)
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
