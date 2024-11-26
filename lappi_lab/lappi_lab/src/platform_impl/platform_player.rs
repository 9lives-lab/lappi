use std::cell::{Cell, RefCell};
use std::io::BufReader;
use std::sync::Arc;
use std::time::Duration;

use lappi_core::platform_api::PlayerApi;
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};

use lappi_core::playback::{Player, PlayerState};
use lappi_core::playback::sources::{SourceType, PlaybackSource};

pub struct PlatformPlayer {
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    sink: RefCell<Sink>,
    current_duration: Cell<Option<Duration>>,
    is_playing: Cell<bool>,
}

impl PlatformPlayer {
    pub fn create() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink: Sink = Sink::try_new(&stream_handle).unwrap();

        Self {
            _stream: stream,
            _stream_handle: stream_handle,
            sink: RefCell::new(sink),
            current_duration: Cell::new(None),
            is_playing: Cell::new(false),
        }
    }
}

impl Player for PlatformPlayer{
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

pub struct DesktopPlayerApi {
    
}

impl PlayerApi for DesktopPlayerApi {
    fn create_platform_player(&self) -> Box<dyn Player> {
        return Box::new(PlatformPlayer::create());
    }
}

pub fn initialize() -> Arc<DesktopPlayerApi> {
    let api = DesktopPlayerApi {

    };
    return Arc::new(api);
}
