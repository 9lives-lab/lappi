pub mod playlist_queue;

use crate::{collection::pictures::PictureId, playback::sources::PlaybackSource};

pub trait PlayQueue: Send + Sync {
    fn get_current_source(&self) -> Box<PlaybackSource>;
    fn get_current_title(&self) -> &str;
    fn get_current_cover(&self) -> Option<PictureId>;
    fn has_next(&self) -> bool;
    fn has_previous(&self) -> bool;
    fn switch_to_next(&mut self);
    fn switch_to_previous(&mut self);
    fn refresh(&mut self);
}

pub struct SingleSourceQueue {
    source: Box<PlaybackSource>,
}

impl SingleSourceQueue {

    pub fn new(source: Box<PlaybackSource>) -> Self {
        Self {
            source,
        }
    }

}

impl PlayQueue for SingleSourceQueue {

    fn get_current_source(&self) -> Box<PlaybackSource> {
        self.source.clone()
    }

    fn get_current_title(&self) -> &str {
        return self.source.get_name()
    }

    fn get_current_cover(&self) -> Option<PictureId> {
        return self.source.get_cover_picture();
    }

    fn has_next(&self) -> bool {
        false
    }

    fn has_previous(&self) -> bool {
        false
    }

    fn switch_to_next(&mut self) {
        unreachable!()
    }

    fn switch_to_previous(&mut self) {
        unreachable!()
    }

    fn refresh(&mut self) {

    }
}
