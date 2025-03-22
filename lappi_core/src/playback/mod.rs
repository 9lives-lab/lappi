pub mod players;
pub mod play_queue;
pub mod sources;
pub mod events;

use std::collections::HashMap;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TryRecvError};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, RwLock};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use amina_core::events::EventEmitter;
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Context, Service, ServiceApi, ServiceInitializer};
use amina_core::tasks::{TaskContext, TaskManager};

use crate::collection::music::MusicItemId;
use crate::collection::pictures::PicturesCollection;
use crate::collection::playlists::types::{PlaylistId, PlaylistItemId};
use crate::collection::OnCollectionUpdated;
use crate::platform_api::PlatformApi;
use crate::playback::events::OnStateUpdated;

use sources::PlaybackSource;
use play_queue::{PlayQueue, SingleSourceQueue};
use play_queue::playlist_queue::PlaylistQueue;
use players::vlc_http::VlcHttpPlayerFactory;
use players::web_player::WebPlayerFactory;

#[derive(Debug, Clone)]
enum PlayerCommand {
    SwitchPlayer(String),
    Play(Box<PlaybackSource>),
    Pause,
    Resume,
    Seek(f32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerState {
    Playing(f32),
    Paused(f32),
    Stopped,
    PlaybackFinished
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerDesc {
    pub id: String,
    pub name: String,
}

pub trait Player {
    fn get_name(&self) -> &str;
    fn play(&self, source: Box<sources::PlaybackSource>);
    fn resume(&self);
    fn pause(&self);
    fn seek(&self, progress: f32);
    fn get_state(&self) -> PlayerState;
}

pub trait PlayerFactory: Send + Sync {
    fn get_name(&self) -> String;
    fn create_player(&self) -> Result<Box<dyn Player>>;
}

pub struct Playback {
    event_emitter: Service<EventEmitter>,
    commands_sender: SyncSender<PlayerCommand>,
    player_factories: RwLock<HashMap<String, Box<dyn PlayerFactory>>>,
    player_state: Arc<RwLock<PlayerState>>,
    current_queue: Mutex<Option<Box<dyn PlayQueue>>>
}

impl Playback {
    pub fn switch_player(&self, player_id: String) {
        self.commands_sender.send(PlayerCommand::SwitchPlayer(player_id)).unwrap();
    }

    pub fn get_players_list(&self) -> Vec<PlayerDesc> {
        let player_factories = self.player_factories.read().unwrap();
        player_factories.keys().map(|id| PlayerDesc {
            id: id.clone(),
            name: player_factories.get(id).unwrap().get_name()
        }).collect()
    }

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
        self.commands_sender.send(PlayerCommand::Play(source)).unwrap();
    }

    pub fn resume(&self) {
        self.commands_sender.send(PlayerCommand::Resume).unwrap();
    }

    pub fn pause(&self) {
        self.commands_sender.send(PlayerCommand::Pause).unwrap();
    }

    pub fn toggle(&self) {
        let state = *self.player_state.read().unwrap();
        match state {
            PlayerState::Playing(_) => {
                self.pause();
            },
            PlayerState::Paused(_) => {
                self.resume();
            },
            PlayerState::Stopped => { },
            PlayerState::PlaybackFinished => { },     
        }
    }

    pub fn seek(&self, progress: i32) {
        self.commands_sender.send(PlayerCommand::Seek((progress as f32) / 1000.)).unwrap();
    }

    pub fn play_next(&self) {
        let mut play_queue = self.current_queue.lock().unwrap();
        if let Some(play_queue) = play_queue.as_mut() {
            if play_queue.has_next() {
                play_queue.switch_to_next();
                self.commands_sender.send(PlayerCommand::Play(play_queue.get_current_source())).unwrap();
            }
        }
    }

    pub fn play_previous(&self) {
        let mut play_queue = self.current_queue.lock().unwrap();
        if let Some(play_queue) = play_queue.as_mut() {
            if play_queue.has_previous() {
                play_queue.switch_to_previous();
                self.commands_sender.send(PlayerCommand::Play(play_queue.get_current_source())).unwrap();
            }
        } 
    }

    fn create_defaut_player(&self) -> Box<dyn Player> {
        let player_factories = self.player_factories.read().unwrap();
        for (player_id, factory) in player_factories.iter() {
            let player = factory.create_player();
            match player {
                Ok(player) => {
                    log::info!("Using default player: {}", player_id);
                    return player;
                },
                Err(err) => {
                    log::warn!("Player {} is not available: {:?}", player_id, err);
                }
            }
        }
    
        panic!("Failed to find default player");
    }

    fn update_player_state(&self, player: &dyn Player) -> PlayerState {
        let state = player.get_state();
        let mut player_state = self.player_state.write().unwrap();
        *player_state = state;

        let mut event = OnStateUpdated::default();

        event.current_player_name = player.get_name();

        let mut cover_path = Option::None;

        let queue = self.current_queue.lock().unwrap();
        if let Some(queue) = queue.as_ref() {
            event.title = queue.get_current_title();
            if let Some(cover) = queue.get_current_cover() {
                let pictures = crate::context().get_service::<PicturesCollection>();
                cover_path = Some(pictures.get_picture_path(cover));
            }
        } else {
            event.title = "Playback stopped";
        }

        event.cover_path = cover_path.as_deref();

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
            },
        }
        self.event_emitter.emit_event(&event);

        return state;
    }

    fn run_task(&self, task_context: &TaskContext, cmd_receiver: Receiver<PlayerCommand>) {
        let mut player = self.create_defaut_player();

        while !task_context.is_interrupted() {
            match cmd_receiver.try_recv() {
                Ok(cmd) => {
                    match cmd {
                        PlayerCommand::SwitchPlayer(player_id) => {
                            let player_factories = self.player_factories.read().unwrap();
                            let factory = player_factories.get(&player_id).unwrap();
                            match factory.create_player() {
                                Ok(new_player) => {
                                    player.pause();
                                    player = new_player;
                                    log::debug!("Switched to player {}", player_id);
                                },
                                Err(err) => {
                                    log::error!("Failed to create player {}: {}", player_id, err);
                                }
                            }
                        },
                        PlayerCommand::Play(source) => {
                            log::debug!("Playing source {:?}", source);
                            player.play(source);
                        },
                        PlayerCommand::Pause => {
                            log::debug!("Pausing playback");
                            player.pause();
                        },
                        PlayerCommand::Resume => {
                            log::debug!("Resuming playback");
                            player.resume();
                        }
                        PlayerCommand::Seek(progress) => {
                            log::debug!("Seeking to {}", progress);
                            player.seek(progress);
                        }
                    }
                },
                Err(TryRecvError::Empty) => { },
                Err(TryRecvError::Disconnected) => {
                    log::debug!("Player command channel disconnected");
                    break;
                }
            };

            let state = self.update_player_state(player.as_ref());

            if state == PlayerState::PlaybackFinished {
                self.play_next();
            }

            thread::sleep(Duration::from_millis(200));
        }
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

impl ServiceInitializer for Playback {
    fn initialize(context: &Context) -> Arc<Self> {
        let event_emitter = context.get_service::<EventEmitter>();
        let task_manager = context.get_service::<TaskManager>();
        let rpc = context.get_service::<Rpc>();
        let platform_api = crate::context().get_service::<PlatformApi>();

        let mut player_factories: HashMap<String, Box<dyn PlayerFactory>> = HashMap::new();
        
        let platform_factories = platform_api.playback.get_platform_player_factories();
        player_factories.extend(platform_factories);
        
        player_factories.insert("web".to_string(), Box::new(WebPlayerFactory::new()));
        player_factories.insert("vlc_http".to_string(), Box::new(VlcHttpPlayerFactory::new(context)));

        let (commands_sender, cmd_reciver) = sync_channel(1);
        let player_state = Arc::new(RwLock::new(PlayerState::Stopped));

        let playback = Arc::new(Self {
            event_emitter: event_emitter.clone(),
            player_factories: RwLock::new(player_factories),
            player_state: player_state.clone(),
            commands_sender,
            current_queue: Mutex::new(None),
        });

        register_rpc_handler!(rpc, playback, "lappi.playback.switch_player", switch_player(player_id: String));
        register_rpc_handler!(rpc, playback, "lappi.playback.get_players_list", get_players_list());
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

        let playback_clone = playback.clone();
        task_manager.run(move |task_context| {
            playback_clone.run_task(&task_context, cmd_reciver);
        });

        return playback;
    }
}

