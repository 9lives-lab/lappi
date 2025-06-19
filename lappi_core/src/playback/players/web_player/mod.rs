use std::path::Path;
use std::sync::{Arc, RwLock};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use amina_core::events::EventEmitter;
use amina_core::events::Event;
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Service, ServiceApi, ServiceInitializer};

use crate::playback::sources::{PlaybackSource, SourceType};
use crate::playback::{Player, PlayerFactory, PlayerState};

static WEB_PLAYER_NAME: &str = "Browser";
static FILE_HANDLER_KEY: &str = "lappi.palyback.web";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebPlayerState {
    Playing,
    Paused,
    Stopped,
    PlaybackFinished,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebPlayerCommand {
    Play { file_name: String },
    Pause,
    Resume,
    Seek { progress: f32 },
    Stop,
}

#[derive(Serialize, Deserialize)]
#[derive(Event)]
#[key = "lappi.playback.web.OnWebPlayerCommand"]
pub struct OnWebPlayerCommand {
    pub command: WebPlayerCommand,
}

pub struct WebPlayer {
    web_player_service: Service<WebPlayerService>,
}

impl Player for WebPlayer {
    fn get_name(&self) -> &str {
        WEB_PLAYER_NAME
    }

    fn play(&self, source: Box<PlaybackSource>) {
        match source.get_source_type() {
            SourceType::LocalFile(path) => {
                self.web_player_service.play_file(path).unwrap();
            },
        }
    }

    fn resume(&self) {
        self.web_player_service.resume();
    }

    fn pause(&self) {
        self.web_player_service.pause();
    }

    fn seek(&self, progress: f32) {
        self.web_player_service.seek(progress);
    }

    fn get_state(&self) -> PlayerState {
        return self.web_player_service.get_player_state();
    }
}

impl Drop for WebPlayer {
    fn drop(&mut self) {
        self.web_player_service.stop();
    }
}

impl WebPlayer {
    pub fn new() -> Self {
        let web_player_service: Service<WebPlayerService> = crate::context().get_service();
        WebPlayer {
            web_player_service,
        }
    }
}

pub struct WebPlayerFactory {

}

impl PlayerFactory for WebPlayerFactory {
    fn get_name(&self) -> String {
        WEB_PLAYER_NAME.to_string()
    }

    fn create_player(&self) -> Result<Box<dyn Player>> {
        return Ok(Box::new(WebPlayer::new()));
    }
}

impl WebPlayerFactory {
    pub fn new() -> Self {
        Self {

        }
    }
}

pub struct WebPlayerContext {
    current_file_path: String,
    player_state: PlayerState,
}

pub struct WebPlayerService {
    event_emitter: Service<EventEmitter>,
    state: RwLock<WebPlayerContext>,
}

impl WebPlayerService {
    pub fn play_file(&self, path: &Path) -> Result<()> {
        let file_name = path.file_name()
                .with_context(|| format!("Path '{:?}' has no file name", &path))?
                .to_str()
                .with_context(|| format!("Path '{:?}' is not valid UTF-8 string", &path))?
                .to_string();

        let event = OnWebPlayerCommand {
            command: WebPlayerCommand::Play {
                file_name: file_name.to_string(),
            },
        };

        let mut state = self.state.write().unwrap();
        state.current_file_path = path.as_os_str()
            .to_str()
            .context("Path is not valid UTF-8 string")?
            .to_string();
        state.player_state = PlayerState::Playing(0.);
        drop(state);

        self.event_emitter.emit_event(&event);

        return Ok(());
    }

    pub fn resume(&self) {
        self.event_emitter.emit_event(&OnWebPlayerCommand {
            command: WebPlayerCommand::Resume
        });
    }

    pub fn pause(&self) {
        self.event_emitter.emit_event(&OnWebPlayerCommand {
            command: WebPlayerCommand::Pause
        });
    }

    pub fn seek(&self, progress: f32) {
        self.event_emitter.emit_event(&OnWebPlayerCommand {
            command: WebPlayerCommand::Seek {
                progress,
            }
        });
    }

    pub fn stop(&self) {
        self.event_emitter.emit_event(&OnWebPlayerCommand {
            command: WebPlayerCommand::Stop
        });
    }

    fn get_player_state(&self) -> PlayerState {
        let state = self.state.read().unwrap();
        return state.player_state;
    }

    pub fn on_web_player_state_changed(&self, web_state: WebPlayerState, progress: f32) {
        let mut state = self.state.write().unwrap();
        match web_state {
            WebPlayerState::Playing => {
                state.player_state = PlayerState::Playing(progress);
            }
            WebPlayerState::Paused => {
                state.player_state = PlayerState::Paused(progress);
            }
            WebPlayerState::Stopped => {
                state.player_state = PlayerState::Stopped;
            }
            WebPlayerState::PlaybackFinished => {
                state.player_state = PlayerState::PlaybackFinished;
            }
        }
    }

    pub fn get_audio_binary(&self, _path: &str) -> std::result::Result<Vec<u8>, std::io::Error> {
        let state = self.state.read().unwrap();
        let path = state.current_file_path.clone();
        drop(state);

        let file_content = std::fs::read(path.as_str())?;
        return Ok(file_content);
    }
}

impl ServiceApi for WebPlayerService {
    fn stop(&self) {

    }
}

impl ServiceInitializer for WebPlayerService {
    fn initialize(context: &amina_core::service::Context) -> Arc<Self> {
        let event_emitter = context.get_service::<EventEmitter>();
        let rpc = context.get_service::<Rpc>();

        let web_player_service = Arc::new(Self {
            event_emitter: event_emitter.clone(),
            state: RwLock::new(WebPlayerContext {
                current_file_path: String::default(),
                player_state: PlayerState::Stopped,
            })
        });

        register_rpc_handler!(rpc, web_player_service, "lappi.playback.web.on_web_player_state_changed", on_web_player_state_changed(web_state: WebPlayerState, progress: f32));

        let web_player_service_copy = web_player_service.clone();
        rpc.add_get_file_handler(FILE_HANDLER_KEY, move|path| {
            web_player_service_copy.get_audio_binary(path)
        });

        return web_player_service;
    }
}

