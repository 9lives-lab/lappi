pub mod file_system;
pub mod ftp;

use std::io::{Read, Write};

use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use camino::Utf8Path;

pub trait RemoteStorageAdapter {
    fn list_dir(&mut self, path: &Utf8Path) -> Result<Vec<String>>;
    fn read_file(&mut self, path: &Utf8Path, dest: &mut dyn Write) -> Result<()>;
    fn write_file(&mut self, path: &Utf8Path, src: &mut dyn Read) -> Result<()>;
    fn remove_file(&mut self, path: &Utf8Path) -> Result<()>;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RemoteStorageSettings {
    pub url: String,
    pub user: String,
    pub password: String,
    pub path: String,
}

pub trait RemoteStorageFactory {
    fn get_name(&self) -> &'static str;
    fn connect(&self, settings: &RemoteStorageSettings) -> Result<Box<dyn RemoteStorageAdapter>>;
}

pub struct DummyRemoteStorage {

}

impl DummyRemoteStorage {
    pub fn new() -> Self {
        Self { }
    }
}

impl RemoteStorageAdapter for DummyRemoteStorage {
    fn list_dir(&mut self, _: &Utf8Path) -> Result<Vec<String>> {
        Err(anyhow!("Remote storage is not available"))
    }

    fn read_file(&mut self, _: &Utf8Path, _: &mut dyn Write) -> Result<()> {
        Err(anyhow!("Remote storage is not available"))
    }

    fn write_file(&mut self, _: &Utf8Path, _: &mut dyn Read) -> Result<()> {
        Err(anyhow!("Remote storage is not available"))
    }

    fn remove_file(&mut self, _: &Utf8Path) -> Result<()> {
        Err(anyhow!("Remote storage is not available"))
    }
}