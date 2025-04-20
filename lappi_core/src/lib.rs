pub mod collection;
pub mod database;
pub mod exploring;
pub mod file_manager;
pub mod import;
pub mod metadata;
pub mod playback;
pub mod proto;
pub mod scripting_engine;
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
