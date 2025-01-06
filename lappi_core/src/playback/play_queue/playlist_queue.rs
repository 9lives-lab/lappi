use crate::collection::pictures::PictureId;
use crate::collection::playlists::types::{PlaylistId, PlaylistItemId};
use crate::collection::playlists::PlaylistsCollection;
use crate::playback::sources::PlaybackSource;
use super::PlayQueue;

struct PlaylistQueueEntry {
    playlist_item_id: PlaylistItemId,
    playback_source: Box<PlaybackSource>,
}
pub struct PlaylistQueue {
    playlist_id: PlaylistId,
    current_playlist_item: PlaylistItemId,
    current_idx: usize,
    queue: Vec<PlaylistQueueEntry>,
}

impl PlaylistQueue {
    pub fn create(playlist_id: PlaylistId, playlist_item: PlaylistItemId) -> Self {
        let mut playlist_queue = Self {
            playlist_id,
            current_playlist_item: playlist_item,
            current_idx: 0,
            queue: vec![],
        };

        playlist_queue.refresh();

        return playlist_queue;
    }    
}

impl PlayQueue for PlaylistQueue {
    fn get_current_source(&self) -> Box<PlaybackSource> {
        self.queue[self.current_idx].playback_source.clone()
    }

    fn get_current_title(&self) -> &str {
        self.queue[self.current_idx].playback_source.get_name()
    }

    fn get_current_cover(&self) -> Option<PictureId> {
        self.queue[self.current_idx].playback_source.get_cover_picture()
    }

    fn has_next(&self) -> bool {
        self.current_idx + 1 < self.queue.len()
    }

    fn has_previous(&self) -> bool {
        0 < self.current_idx
    }

    fn switch_to_next(&mut self) {
        if self.has_next() {
            self.current_idx += 1;
            self.current_playlist_item = self.queue[self.current_idx].playlist_item_id;
        }
    }

    fn switch_to_previous(&mut self) {
        if self.has_previous() {
            self.current_idx -= 1;
            self.current_playlist_item = self.queue[self.current_idx].playlist_item_id;
        }
    }

    fn refresh(&mut self) {
        let playlists = crate::context().get_service::<PlaylistsCollection>();

        let mut new_queue = vec![];
        let mut current_idx = 0;

        for item in playlists.get_playlist_items(self.playlist_id) {
            let playback_source = PlaybackSource::default_from_music_item(item.music_item_id);
            if let Some(playback_source) = playback_source {
                if item.id == self.current_playlist_item {
                    current_idx = new_queue.len();
                }

                new_queue.push(PlaylistQueueEntry {
                    playlist_item_id: item.id,
                    playback_source,
                });
            }
        }

        self.queue = new_queue;
        self.current_idx = current_idx;
    }
}

