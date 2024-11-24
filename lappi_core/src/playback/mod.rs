pub mod players;
pub mod play_queue;
pub mod sources;
pub mod events;

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;

use amina_core::events::EventEmitter;
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Context, Service, ServiceApi, ServiceInitializer};
use amina_core::tasks::{Task, TaskManager};
use play_queue::playlist_queue::PlaylistQueue;
use sources::PlaybackSource;

use crate::collection::music::MusicItemId;
use crate::collection::playlists::types::{PlaylistId, PlaylistItemId};
use crate::collection::OnCollectionUpdated;
use crate::playback::events::OnStateUpdated;

use play_queue::{PlayQueue, SingleSourceQueue};

#[derive(Debug)]
pub enum PlayerState {
    Playing(f32),
    Paused(f32),
    Stopped,
    PlaybackFinished
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
    event_emitter: Service<EventEmitter>,
    current_player: Box<dyn Player>,
    current_queue: Mutex<Option<Box<dyn PlayQueue>>>
}

impl Playback {
    pub fn play_item(&self, item_id: MusicItemId) {
        if let Some(source) = PlaybackSource::default_from_music_item(item_id) {
            self.play_queue(Box::new(SingleSourceQueue::new(source)));
        } else {
            log::debug!("No source files for music item {}", item_id);
        }
    }

    pub fn play_playlist(&self, playlist_id: PlaylistId, playlist_item: PlaylistItemId) {
        let play_queue = PlaylistQueue::create(playlist_id, playlist_item);
        self.play_queue(Box::new(play_queue));
    }

    pub fn play_queue(&self, play_queue: Box<dyn PlayQueue>) {
        let source = play_queue.get_current_source();
        self.current_queue.lock().unwrap().replace(play_queue);
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
            PlayerState::PlaybackFinished => { },     
        }
    }

    pub fn seek(&self, progress: i32) {
        self.current_player.seek((progress as f32) / 1000.);
    }

    pub fn play_next(&self) {
        let mut play_queue = self.current_queue.lock().unwrap();
        if let Some(play_queue) = play_queue.as_mut() {
            play_queue.switch_to_next();
            self.current_player.play(play_queue.get_current_source());
        }
    }

    pub fn play_previous(&self) {
        let mut play_queue = self.current_queue.lock().unwrap();
        if let Some(play_queue) = play_queue.as_mut() {
            play_queue.switch_to_previous();
            self.current_player.play(play_queue.get_current_source());
        } 
    }

    fn run_task(&self) {
        let state = self.current_player.get_state();
        let mut event = OnStateUpdated::stopped();

        let queue = self.current_queue.lock().unwrap();
        if let Some(queue) = queue.as_ref() {
            event.title = queue.get_current_title().to_string();
        } else {
            event.title = "Playback stopped".to_string();
        }
        drop(queue);

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
            }
            PlayerState::PlaybackFinished => {
                event.is_playing = true;
                event.progress = 0;
                self.play_next();
            },
        }
        self.event_emitter.emit_event(&event);
    }

    fn on_collection_updated(&self, _event: &OnCollectionUpdated) {
        if let Some(queue) = self.current_queue.lock().unwrap().as_mut() {
            queue.refresh();
        }
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
        let event_emitter = context.get_service::<EventEmitter>();
        let task_manager = context.get_service::<TaskManager>();
        let rpc = context.get_service::<Rpc>();

        let vlc_http_player_factory = players::vlc_http::VlcHttpPlayerFactory::new(context);

        let playback = Arc::new(Self {
            event_emitter: event_emitter.clone(),
            current_player: vlc_http_player_factory.create_player(),
            current_queue: Mutex::new(None),
        });

        register_rpc_handler!(rpc, playback, "lappi.playback.play_item", play_item(item_id: MusicItemId));
        register_rpc_handler!(rpc, playback, "lappi.playback.play_playlist", play_playlist(playlist_id: PlaylistId, playlist_item: PlaylistItemId));
        register_rpc_handler!(rpc, playback, "lappi.playback.toggle", toggle());
        register_rpc_handler!(rpc, playback, "lappi.playback.resume", resume());
        register_rpc_handler!(rpc, playback, "lappi.playback.pause", pause());
        register_rpc_handler!(rpc, playback, "lappi.playback.seek", seek(progress: i32));
        register_rpc_handler!(rpc, playback, "lappi.playback.play_next", play_next());
        register_rpc_handler!(rpc, playback, "lappi.playback.play_previous", play_previous());

        let playback_clone = playback.clone();
        event_emitter.on_event_fn(move |event: &OnCollectionUpdated| {
            playback_clone.on_collection_updated(event);
        });

        let playback_task = PlaybackTask::new(playback.clone());
        task_manager.run_generic(Box::new(playback_task));

        return playback;
    }
}

