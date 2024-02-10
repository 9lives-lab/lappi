pub mod py_source;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use serde::Serialize;
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Context, ServiceApi, ServiceInitializer};

use crate::exploring::py_source::PyExploringSource;
use crate::py_server_client::PyServerClient;
use crate::settings::Settings;

#[derive(Serialize)]
pub enum ExploringError {
    NoSource,
    ConnectionError,
    RequestError(String),
}

pub type ExploringResult<T> = Result<T, ExploringError>;

pub trait ExploringSource: Send + Sync {
    fn source_name(&self) -> String;
    fn get_artist_description(&self, artist_name: &str) -> ExploringResult<String>;
    fn clone_source(&self) -> Box<dyn ExploringSource>;
}

pub struct Exploring {

    sources: RwLock<HashMap<String, Box<dyn ExploringSource>>>
}

impl Exploring {

    pub fn get_source_list(&self) -> Vec<String> {
        let source_list = self.sources.read().unwrap();
        return source_list.keys().map(|key| key.clone()).collect();
    }

    pub fn get_source(&self, source: &str) -> ExploringResult<Box<dyn ExploringSource>> {
        let source_list = self.sources.read().unwrap();
        return source_list
            .get(source)
            .map(|source| source.clone_source())
            .ok_or(ExploringError::NoSource);
    }

    fn add_source(&self, source: Box<dyn ExploringSource>) {
        let mut source_list = self.sources.write().unwrap();
        source_list.insert(source.source_name(), source);
    }

    pub fn get_artist_description(&self, source_name: String, artist_name: String) -> ExploringResult<String> {
        let source = self.get_source(&source_name)?;
        return source.get_artist_description(&artist_name);
    }

}

impl ServiceApi for Exploring {

}

impl ServiceInitializer for Exploring {

    fn initialize(context: &Context) -> Arc<Self> {
        let settings = context.get_service::<Settings>();
        let py_server_client = context.get_service::<PyServerClient>();
        let rpc = context.get_service::<Rpc>();

        let exploring_service = Arc::new(Exploring {
            sources: RwLock::new(HashMap::new()),
        });

        if py_server_client.is_connected() {
            let discogs_user_token = settings.get_string("exploring.sources.discogs.user_token").get();
            py_server_client.set_registry_value("exploring.sources.discogs.user_token", discogs_user_token).unwrap();
            exploring_service.add_source(Box::new(PyExploringSource::new("discogs.com".to_string(), py_server_client.clone())));

            let chatgpt_user_token = settings.get_string("exploring.sources.chatgpt.user_token").get();
            py_server_client.set_registry_value("exploring.sources.chatgpt.user_token", chatgpt_user_token).unwrap();
            exploring_service.add_source(Box::new(PyExploringSource::new("chatgpt".to_string(), py_server_client)));
        }

        register_rpc_handler!(rpc, exploring_service, "lappi.exploring.get_source_list", get_source_list());
        register_rpc_handler!(rpc, exploring_service, "lappi.exploring.get_artist_description", get_artist_description(source_name: String, artist_name: String));

        return exploring_service;
    }

}