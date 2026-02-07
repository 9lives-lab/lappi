use std::{cell::Cell, io::{Read, Write}};

use anyhow::{Context, Result};
use camino::Utf8Path;
use suppaftp::FtpStream;

use crate::storage::remote::adapter::{RemoteStorageAdapter, RemoteStorageFactory, RemoteStorageSettings};

pub struct FtpAdapter {
    stream: Cell<FtpStream>,
}

impl RemoteStorageAdapter for FtpAdapter {
    fn list_dir(&mut self, path: &Utf8Path) -> Result<Vec<String>> {
        let stream = self.stream.get_mut();
        let list = stream.list(Some(path.as_str()))?;
        Ok(list)
    }

    fn read_file(&mut self, path: &Utf8Path, dest: &mut dyn Write) -> Result<()> {
        let stream = self.stream.get_mut();
        let parent = path.parent().context(format!("Path has no parent folder '{:?}'", path))?;
        let file_name = path.file_name().context(format!("Path has no file name '{:?}'", path))?;
        stream.cwd(parent)?;
        let mut reader = stream.retr_as_stream(file_name)?;
        std::io::copy(&mut reader, dest)?;
        stream.finalize_retr_stream(reader)?;
        Ok(())
    }

    fn write_file(&mut self, path: &Utf8Path, src: &mut dyn Read) -> Result<()> {
        let stream = self.stream.get_mut();
        let parent = path.parent().context(format!("Path has no parent folder '{:?}'", path))?;
        let filename = path.file_name().context(format!("Path has no file name '{:?}'", path))?;
        stream.cwd(parent)?;
        let mut writer = stream.put_with_stream(filename)?;
        std::io::copy(src, &mut writer)?;
        stream.finalize_retr_stream(writer)?;
        Ok(())
    }

    fn remove_file(&mut self, path: &Utf8Path) -> Result<()> {
        let stream = self.stream.get_mut();
        let file_path = path.as_str();
        stream.rm(file_path).context(format!("Failed to delete file '{}'", file_path))
    }
}

impl Drop for FtpAdapter {
    fn drop(&mut self) {
        let stream = self.stream.get_mut();
        let _ = stream.quit();
    }
}
pub struct FtpStorageFactory {

}

impl FtpStorageFactory {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl RemoteStorageFactory for FtpStorageFactory {
    fn get_name(&self) -> &'static str {
        "FTP"
    }

    fn connect(&self, settings: &RemoteStorageSettings) -> Result<Box<dyn RemoteStorageAdapter>> {
        let mut ftp_stream = FtpStream::connect(&settings.url)?;
        ftp_stream.login(&settings.user, &settings.password)?;
        Ok(Box::new(FtpAdapter {
            stream: Cell::new(ftp_stream)
        }))   
    }
}

