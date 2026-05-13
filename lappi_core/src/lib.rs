use crate::storage::local::LocalStorage;
use crate::storage::remote::RemoteStorage;
use crate::playback::players::web_player::WebPlayerService;
use crate::playback::Playback;
use crate::database::Database;
use crate::exploring::chat::ChatService;
use crate::exploring::chat::templates::ChatTemplates;
use crate::import::collection::CollectionImporter;
use crate::scripting_engine::ScriptingEngine;
use crate::settings::Settings;
use crate::exploring::lyrics::LyricsExplorer;
use crate::workspace::Workspace;
use crate::jobs::Jobs;
use crate::file_manager::FileManager;
use crate::file_manager::search::FilesExplorer;
use crate::py_server_client::PyServerClient;

pub mod collection;
pub mod database;
pub mod exploring;
pub mod file_manager;
pub mod import;
pub mod metadata;
pub mod playback;
pub mod proto;
pub mod jobs;
pub mod scripting_engine;
pub mod storage;
pub mod utils;
pub mod workspace;

pub mod app_config;
pub mod platform_api;
pub mod py_server_client;
pub mod settings;

pub mod ui;

#[macro_use]
extern crate num_derive;

#[macro_use]
extern crate amina_core_derive;

use once_cell::sync::Lazy;
use amina_core::service::Context;

pub fn context() -> &'static Context {
    static INSTANCE: Lazy<Context> = Lazy::new(|| {
        Context::new()
    });
    &INSTANCE
}

pub fn initilaize() {
    let context = context();

    context.init_service::<Workspace>();
    context.init_service::<Settings>();
    context.init_service::<Jobs>();
    context.init_service::<ScriptingEngine>();
    context.init_service::<FileManager>();
    context.init_service::<FilesExplorer>();
    context.init_service::<Database>();
    context.init_service::<LocalStorage>();
    context.init_service::<RemoteStorage>();

    crate::collection::initialize();

    context.init_service::<WebPlayerService>();
    context.init_service::<Playback>();
    context.init_service::<CollectionImporter>();
    context.init_service::<PyServerClient>();
    context.init_service::<ChatService>();
    context.init_service::<ChatTemplates>();
    context.init_service::<LyricsExplorer>();
}

