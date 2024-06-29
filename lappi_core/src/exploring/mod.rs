pub mod lyrics;
pub mod chat;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ExploringError {
    NoSource,
    ConnectionError,
    RequestError(String),
    GenericError(String),
}

impl From<reqwest::Error> for ExploringError {
    fn from(value: reqwest::Error) -> Self {
        ExploringError::RequestError(value.to_string())
    }
}

pub type ExploringResult<T> = Result<T, ExploringError>;

pub trait ExploringSource: Send + Sync {
    fn source_name(&self) -> &str;
}

pub struct SourceList<T: ExploringSource> {
    sources: RwLock<HashMap<String, Arc<T>>>,
}

impl <T: ExploringSource> SourceList<T> {
    pub fn new() -> Self {
        Self {
            sources: RwLock::new(HashMap::new())
        }
    }

    pub fn get_sources(&self) -> Vec<Arc<T>> {
        let source_list = self.sources.read().unwrap();
        return source_list.values().map(|s| s.clone()).collect();
    }

    pub fn get_source_list(&self) -> Vec<String> {
        let source_list = self.sources.read().unwrap();
        return source_list.keys().cloned().collect();
    }

    pub fn get_source(&self, source: &str) -> ExploringResult<Arc<T>> {
        let source_list = self.sources.read().unwrap();
        return source_list.get(source).cloned().ok_or(ExploringError::NoSource);
    }

    pub fn add_source(&self, source: T) {
        let mut source_list = self.sources.write().unwrap();
        source_list.insert(source.source_name().to_string(), Arc::new(source));
    }
}
