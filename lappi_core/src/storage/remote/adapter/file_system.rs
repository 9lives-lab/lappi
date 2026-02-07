use std::io::{Read, Write};

use anyhow::{Context, Result};
use camino::{Utf8Path, Utf8PathBuf};

use crate::storage::remote::adapter::{RemoteStorageAdapter, RemoteStorageFactory, RemoteStorageSettings};

pub struct FileSystemAdapter {
    base_path: Utf8PathBuf,
}

impl FileSystemAdapter {
    fn ensure_directory_exists(&self, path: &Utf8PathBuf) -> Result<()> {
        let parent_path = path.parent().ok_or_else(|| anyhow::anyhow!("Failed to get parent directory of {:?}", path))?;
        if !parent_path.exists() {
            std::fs::create_dir_all(parent_path).context(format!("Failed to create directory {:?}", parent_path))?;
        }
        Ok(())
    }
}

impl RemoteStorageAdapter for FileSystemAdapter {
    fn list_dir(&mut self, path: &Utf8Path) -> Result<Vec<String>> {
        let full_path = self.base_path.join(path);
        std::fs::read_dir(full_path)
            .context(format!("Failed to list directory {:?}", path))
            .map(|read_dir| {
                read_dir.filter_map(|entry| entry.ok())
                        .map(|entry| entry.file_name().to_string_lossy().into_owned())
                        .collect()
            })
    }

    fn read_file(&mut self, path: &Utf8Path, dest: &mut dyn Write) -> Result<()> {
        let full_path = self.base_path.join(path);
        let mut file = std::fs::File::open(full_path).context(format!("Failed to open file {:?}", path))?;
        std::io::copy(&mut file, dest).context("Failed to read file")?;
        Ok(())
    }

    fn write_file(&mut self, path: &Utf8Path, src: &mut dyn Read) -> Result<()> {
        let full_path = self.base_path.join(path);
        self.ensure_directory_exists(&full_path)?;
        let mut file = std::fs::File::create(full_path).context(format!("Failed to create file {:?}", path))?;
        std::io::copy(src, &mut file).context("Failed to write file")?;
        Ok(())
    }

    fn remove_file(&mut self, path: &Utf8Path) -> Result<()> {
        let full_path = self.base_path.join(path);
        std::fs::remove_file(&full_path).context(format!("Failed to remove file {:?}", full_path))?;
        Ok(())
    }
}
pub struct FileSystemFactory {

}

impl FileSystemFactory {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl RemoteStorageFactory for FileSystemFactory {
    fn get_name(&self) -> &'static str {
        "File system"
    }

    fn connect(&self, settings: &RemoteStorageSettings) -> Result<Box<dyn RemoteStorageAdapter>> {
        Ok(Box::new(FileSystemAdapter {
            base_path: Utf8PathBuf::from(&settings.url),
        }))   
    }
}

