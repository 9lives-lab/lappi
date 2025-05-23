use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::io::BufReader;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};

use lappi_core::platform_api::PlaybackApi;
use lappi_core::playback::{Player, PlayerFactory, PlayerState};
use lappi_core::playback::sources::{SourceType, PlaybackSource};

static NATIVE_PLAYER_NAME: &str = "Current device";

pub struct NativePlayer {
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    sink: RefCell<Sink>,
    current_duration: Cell<Option<Duration>>,
    is_playing: Cell<bool>,
}

impl NativePlayer {
    pub fn create() -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink: Sink = Sink::try_new(&stream_handle).unwrap();

        Ok(Self {
            _stream: stream,
            _stream_handle: stream_handle,
            sink: RefCell::new(sink),
            current_duration: Cell::new(None),
            is_playing: Cell::new(false),
        })
    }
}

impl Player for NativePlayer{
    fn get_name(&self) -> &str {
        NATIVE_PLAYER_NAME
    }

    fn play(&self, source: Box<PlaybackSource>) {
        match source.get_source_type() {
            SourceType::LocalFile(path) => {
                let file = std::fs::File::open(path).unwrap();
                let decoder = rodio::Decoder::new(BufReader::new(file)).unwrap();
                let duration = decoder.total_duration();
                let sink = &self.sink.borrow();
                sink.clear();
                sink.append(decoder);
                sink.play();
                self.current_duration.set(duration);
                self.is_playing.set(true);
            },
        }
    }

    fn resume(&self) {
        self.sink.borrow().play();
        self.is_playing.set(true);
    }

    fn pause(&self) {
        self.sink.borrow().pause();
        self.is_playing.set(false);
    }

    fn seek(&self, progress: f32) {
        if let Some(duration) = self.current_duration.get() {
            let sink = self.sink.borrow();
            let position = duration.mul_f32(progress);
            sink.try_seek(position).unwrap();
        }
    }

    fn get_state(&self) -> PlayerState {
        let sink = self.sink.borrow();
        if sink.empty() {
            if self.is_playing.get() == true {
                return PlayerState::PlaybackFinished;
            } else {
                return PlayerState::Stopped;
            }
        } else {
            let position = match self.current_duration.get() {
                Some(duration) => {
                    let position = sink.get_pos().as_secs_f32();
                    position / duration.as_secs_f32() 
                },
                None => 0f32
            };
        
            if sink.is_paused() {
                return PlayerState::Paused(position);
            } else {
                return PlayerState::Playing(position);
            }
        }

    }
}

pub struct NativePlayerFactory {

}

impl PlayerFactory for NativePlayerFactory {
    fn get_name(&self) -> String {
        NATIVE_PLAYER_NAME.to_string()
    }

    fn create_player(&self) -> Result<Box<dyn Player>> {
        return Ok(Box::new(NativePlayer::create()?));
    }
}

impl NativePlayerFactory {
    pub fn new() -> Self {
        Self {

        }
    }
}

pub struct PlatformPlaybackApi {
    
}

impl PlaybackApi for PlatformPlaybackApi {
    fn get_platform_player_factories(&self) -> HashMap<String, Box<dyn PlayerFactory>> {
        let mut factories: HashMap<String, Box<dyn PlayerFactory>> = HashMap::new();

        let enable_native_player: bool = std::env::var("LAPPI_ENABLE_NATIVE_PLAYER")
            .unwrap_or("true".to_string())
            .parse()
            .unwrap();

        if enable_native_player {
            factories.insert("native".to_string(), Box::new(NativePlayerFactory::new()));
        }
        
        return factories;
    }
}

pub fn initialize() -> Arc<PlatformPlaybackApi> {
    let api = PlatformPlaybackApi {

    };
    return Arc::new(api);
}
