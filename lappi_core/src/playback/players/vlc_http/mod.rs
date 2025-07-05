use std::cell::Cell;

use anyhow::Result;
use amina_core::service::{Context, Service};

use crate::playback::{Player, PlayerFactory, PlayerState};
use crate::playback::sources::{PlaybackSource, SourceType};
use crate::settings::Settings;

pub mod http_api;

static VLC_HTTP_PLAYER_NAME: &str = "VLC Remote";

pub struct VlcHttpPlayer {
    api: http_api::VlcHttpApi,
    current_length: Cell<i32>,
    is_playing: Cell<bool>,
}

impl Player for VlcHttpPlayer {
    fn get_name(&self) -> &str {
        VLC_HTTP_PLAYER_NAME
    }

    fn play(&self, source: Box<PlaybackSource>) {
        match source.get_source_type() {
            SourceType::LocalFile(path) => {
                let _ = self.api.play_file(&path);
                self.is_playing.set(true);
            },
        }
    }

    fn resume(&self) {
        let _ = self.api.resume();
        self.is_playing.set(true);
    }

    fn pause(&self) {
        let _ = self.api.pause();
        self.is_playing.set(false);
    }

    fn seek(&self, progress: f32) {
        self.get_state();
        let length = self.current_length.get();
        let vlc_progress = (length as f32 * progress) as i32;
        log::debug!("Seeking to {} - {}/{}", progress, vlc_progress, length);
        let _ = self.api.seek(vlc_progress);
    }

    fn get_state(&self) -> PlayerState {
        let status = self.api.get_status();
        match status {
            Ok(status) => {
                self.current_length.set(status.length);
                match status.state.as_str() {
                    "playing" => PlayerState::Playing(status.position as f32),
                    "paused" => PlayerState::Paused(status.position as f32),
                    "stopped" => {
                        if self.is_playing.get() {
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
            current_length: Cell::new(0),
            is_playing: Cell::new(false),
        }
    }
}

pub struct VlcHttpPlayerFactory {
    settings: Service<Settings>,
}

impl PlayerFactory for VlcHttpPlayerFactory {
    fn get_name(&self) -> String {
        VLC_HTTP_PLAYER_NAME.to_string()
    }

    fn create_player(&self) -> Result<Box<dyn Player>> {
        Ok(Box::new(VlcHttpPlayer::new(self.settings.clone())))
    }
}

impl VlcHttpPlayerFactory {
    pub fn new(context: &Context) -> Self {
        VlcHttpPlayerFactory {
            settings: context.get_service::<Settings>(),
        }
    }
}
