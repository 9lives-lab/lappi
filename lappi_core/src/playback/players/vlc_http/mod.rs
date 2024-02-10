use std::ops::Deref;
use std::path::Path;
use std::sync::atomic::AtomicI32;

use crate::playback::{Player, PlayerState};
use crate::playback::sources::PlaybackSource;

pub mod http_api;

pub struct VlcHttpPlayer {
    api: http_api::VlcHttpApi,
    current_length: AtomicI32,
}

impl Player for VlcHttpPlayer {

    fn play(&self, source: Box<PlaybackSource>) {
        match source.deref() {
            PlaybackSource::LocalFile(path) => {
                let _ = self.api.play_file(Path::new(path));
            },
        }
    }

    fn resume(&self) {
        let _ = self.api.resume();
    }

    fn pause(&self) {
        let _ = self.api.pause();
    }

    fn seek(&self, progress: f32) {
        let length = self.current_length.load(std::sync::atomic::Ordering::Relaxed);
        let _ = self.api.seek((length as f32 * progress) as i32);
    }

    fn get_state(&self) -> PlayerState {
        let status = self.api.get_status();
        match status {
            Ok(status) => {
                self.current_length.store(status.length, std::sync::atomic::Ordering::Relaxed);
                match status.state.as_str() {
                    "playing" => PlayerState::Playing(status.position as f32),
                    "paused" => PlayerState::Paused(status.position as f32),
                    "stopped" => PlayerState::Stopped,
                    _ => panic!("Unknown state: {}", status.state),
                }
            },
            Err(_) => PlayerState::Stopped,
        }
    }

}

impl VlcHttpPlayer {
    pub fn new() -> Self {
        VlcHttpPlayer {
            api: http_api::VlcHttpApi::new(),
            current_length: AtomicI32::new(0),
        }
    }
}

