pub mod lyrics;
pub mod chat;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use anyhow::{Context, Result};

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
        source_list.values()
            .map(|s| s.clone())
            .collect()
    }

    pub fn get_source_list(&self) -> Vec<String> {
        let source_list = self.sources.read().unwrap();
        source_list.keys()
            .cloned()
            .collect()
    }

    pub fn get_source(&self, source: &str) -> Result<Arc<T>> {
        let source_list = self.sources.read().unwrap();
        source_list.get(source)
           .cloned()
           .context(format!("No source found '{}'", source))
    }

    pub fn add_source(&self, source: T) {
        let mut source_list = self.sources.write().unwrap();
        source_list.insert(source.source_name().to_string(), Arc::new(source));
    }
}
