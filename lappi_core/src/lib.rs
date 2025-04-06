pub mod collection;
pub mod database;
pub mod metadata;
pub mod playback;
pub mod workspace;
pub mod file_manager;
pub mod import;
pub mod exploring;
pub mod py_server_client;
pub mod scripting_engine;

pub mod app_config;
pub mod platform_api;
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
