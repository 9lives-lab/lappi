use std::ops::Deref;
use crate::playback::sources::PlaybackSource;

pub trait Playlist: Send + Sync {
    fn get_current_source(&self) -> Box<PlaybackSource>;
    fn get_current_title(&self) -> String;
    fn has_next(&self) -> bool;
    fn has_previous(&self) -> bool;
    fn switch_to_next(&self);
    fn switch_to_previous(&self);
}

pub struct SingleSourcePlaylist {
    source: Box<PlaybackSource>,
}

impl SingleSourcePlaylist {

    pub fn new(source: Box<PlaybackSource>) -> Self {
        SingleSourcePlaylist {
            source,
        }
    }

}

impl Playlist for SingleSourcePlaylist {

    fn get_current_source(&self) -> Box<PlaybackSource> {
        self.source.clone()
    }

    fn get_current_title(&self) -> String {
        match self.source.deref() {
            PlaybackSource::LocalFile(path) => {
                path.clone()
            },
        }
    }

    fn has_next(&self) -> bool {
        false
    }

    fn has_previous(&self) -> bool {
        false
    }

    fn switch_to_next(&self) {
        unreachable!()
    }

    fn switch_to_previous(&self) {
        unreachable!()
    }

}
