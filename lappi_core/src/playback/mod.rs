pub mod players;
pub mod playlists;
pub mod sources;
pub mod events;

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool};

use amina_core::events::EventEmitter;
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Context, Service, ServiceApi, ServiceInitializer};
use amina_core::tasks::{Task, TaskManager};

use crate::collection::Collection;
use crate::collection::types::ItemId;
use crate::playback::events::OnStateUpdated;
use crate::playback::playlists::Playlist;

#[derive(Debug)]
pub enum PlayerState {
    Playing(f32),
    Paused(f32),
    Stopped,
}

pub trait Player: Send + Sync {
    fn play(&self, source: Box<sources::PlaybackSource>);
    fn resume(&self);
    fn pause(&self);
    fn seek(&self, progress: f32);
    fn get_state(&self) -> PlayerState;
}

pub trait PlayerFactory {
    fn create_player(&self) -> Box<dyn Player>;
}

pub struct Playback {
    collection: Service<Collection>,
    event_emitter: Service<EventEmitter>,
    current_player: Box<dyn Player>,
    current_playlist: Mutex<Option<Box<dyn Playlist>>>
}

impl Playback {
    pub fn play_item(&self, item_id: ItemId) {
        let file = self.collection.music().get_external_src_files(item_id).get(0).unwrap().path.clone();
        let source = Box::new(sources::PlaybackSource::LocalFile(file.clone()));
        let playlist = Box::new(playlists::SingleSourcePlaylist::new(source));
        self.play_playlist(playlist);
    }

    pub fn play_playlist(&self, playlist: Box<dyn Playlist>) {
        let source = playlist.get_current_source();
        self.current_playlist.lock().unwrap().replace(playlist);
        self.current_player.play(source);
    }

    pub fn resume(&self) {
        self.current_player.resume();
    }

    pub fn pause(&self) {
        self.current_player.pause();
    }

    pub fn toggle(&self) {
        let state = self.current_player.get_state();
        match state {
            PlayerState::Playing(_) => {
                self.current_player.pause();
            },
            PlayerState::Paused(_) => {
                self.current_player.resume();
            },
            PlayerState::Stopped => { },
        }
    }

    pub fn seek(&self, progress: i32) {
        self.current_player.seek((progress as f32) / 1000.);
    }

    pub fn play_next(&self) {
        todo!()
    }

    pub fn play_previous(&self) {
        todo!()
    }

    fn run_task(&self) {
        let state = self.current_player.get_state();
        let mut event = OnStateUpdated::stopped();

        let playlist = self.current_playlist.lock().unwrap();
        if let Some(playlist) = playlist.as_ref() {
            event.title = playlist.get_current_title();
        } else {
            event.title = "Playback stopped".to_string();
        }

        match state {
            PlayerState::Playing(position) => {
                event.is_playing = true;
                event.progress = (position * 1000.) as i32;
            },
            PlayerState::Paused(position) => {
                event.is_playing = false;
                event.progress = (position * 1000.) as i32;
            },
            PlayerState::Stopped => {
                event.is_playing = false;
                event.progress = 0;
            },
        }
        self.event_emitter.emit_event(&event);
    }
}

impl ServiceApi for Playback {
    fn stop(&self) {

    }
}

struct PlaybackTask {
    playback: Arc<Playback>,
    is_interrupted: AtomicBool,
}

impl PlaybackTask {
    pub fn new(playback: Arc<Playback>) -> Self {
        Self {
            playback,
            is_interrupted: AtomicBool::new(false),
        }
    }
}

impl Task for PlaybackTask {
    fn run(&self) {
        while !self.is_interrupted.load(std::sync::atomic::Ordering::Relaxed) {
            self.playback.run_task();
            thread::sleep(Duration::from_millis(1000));
        }
    }

    fn stop(&self) {
        self.is_interrupted.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

impl ServiceInitializer for Playback {
    fn initialize(context: &Context) -> Arc<Self> {
        let collection = context.get_service::<Collection>();
        let event_emitter = context.get_service::<EventEmitter>();
        let task_manager = context.get_service::<TaskManager>();
        let rpc = context.get_service::<Rpc>();

        let vlc_http_player_factory = players::vlc_http::VlcHttpPlayerFactory::new(context);

        let playback = Arc::new(Self {
            collection,
            event_emitter,
            current_player: vlc_http_player_factory.create_player(),
            current_playlist: Mutex::new(None),
        });

        register_rpc_handler!(rpc, playback, "lappi.playback.play_item", play_item(item_id: ItemId));
        register_rpc_handler!(rpc, playback, "lappi.playback.toggle", toggle());
        register_rpc_handler!(rpc, playback, "lappi.playback.resume", resume());
        register_rpc_handler!(rpc, playback, "lappi.playback.pause", pause());
        register_rpc_handler!(rpc, playback, "lappi.playback.seek", seek(progress: i32));
        register_rpc_handler!(rpc, playback, "lappi.playback.play_next", play_next());
        register_rpc_handler!(rpc, playback, "lappi.playback.play_previous", play_previous());

        let playback_task = PlaybackTask::new(playback.clone());
        task_manager.run_generic(Box::new(playback_task));

        return playback;
    }
}

