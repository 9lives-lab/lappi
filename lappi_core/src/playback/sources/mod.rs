use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum SourceType {
    LocalFile(String),
}

#[derive(Clone, Debug)]
pub struct PlaybackSource {
    pub name: String,
    pub source_type: SourceType,
}

impl PlaybackSource {
    pub fn local_file(name: String, path: String) -> Box<PlaybackSource> {
        Box::new(Self {
            name,
            source_type: SourceType::LocalFile(path),
        })
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_source_type(&self) -> &SourceType {
        &self.source_type
    }
}
