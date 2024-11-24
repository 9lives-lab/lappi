use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use amina_core::service::{Context, Service};

use crate::playback::{Player, PlayerFactory, PlayerState};
use crate::playback::sources::{PlaybackSource, SourceType};
use crate::settings::Settings;

pub mod http_api;

pub struct VlcHttpPlayer {
    api: http_api::VlcHttpApi,
    current_length: AtomicI32,
    is_playing: AtomicBool,
}

impl Player for VlcHttpPlayer {

    fn play(&self, source: Box<PlaybackSource>) {
        match source.get_source_type() {
            SourceType::LocalFile(path) => {
                let _ = self.api.play_file(Path::new(path));
                self.is_playing.store(true, Ordering::Relaxed);
            },
        }
    }

    fn resume(&self) {
        let _ = self.api.resume();
        self.is_playing.store(true, Ordering::Relaxed);
    }

    fn pause(&self) {
        let _ = self.api.pause();
        self.is_playing.store(false, Ordering::Relaxed);
    }

    fn seek(&self, progress: f32) {
        let length = self.current_length.load(Ordering::Relaxed);
        let _ = self.api.seek((length as f32 * progress) as i32);
    }

    fn get_state(&self) -> PlayerState {
        let status = self.api.get_status();
        match status {
            Ok(status) => {
                self.current_length.store(status.length, Ordering::Relaxed);
                match status.state.as_str() {
                    "playing" => PlayerState::Playing(status.position as f32),
                    "paused" => PlayerState::Paused(status.position as f32),
                    "stopped" => {
                        if self.is_playing.load(Ordering::Relaxed) {
                            PlayerState::PlaybackFinished
                        } else {
                            PlayerState::Stopped
                        }
                    }
                    _ => panic!("Unknown state: {}", status.state),
                }
            },
            Err(_) => PlayerState::Stopped,
        }
    }

}

impl VlcHttpPlayer {
    pub fn new(settings: Service<Settings>) -> Self {
        VlcHttpPlayer {
            api: http_api::VlcHttpApi::new(settings),
            current_length: AtomicI32::new(0),
            is_playing: AtomicBool::new(false),
        }
    }
}

pub struct VlcHttpPlayerFactory {
    settings: Service<Settings>,
}

impl PlayerFactory for VlcHttpPlayerFactory {
    fn create_player(&self) -> Box<dyn Player> {
        Box::new(VlcHttpPlayer::new(self.settings.clone()))
    }
}

impl VlcHttpPlayerFactory {
    pub fn new(context: &Context) -> Self {
        VlcHttpPlayerFactory {
            settings: context.get_service::<Settings>(),
        }
    }
}
