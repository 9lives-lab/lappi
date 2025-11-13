use serde::{Serialize, Deserialize};

pub type InternalFileId = i64;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InternalPath {
    path: String,
}

impl InternalPath {
    pub fn new() -> Self {
        Self { 
            path: String::new()
        }
    }

    pub fn from_string(path: String) -> Self {
        Self { 
            path
        }
    }

    pub fn push(&mut self, folder_name: &str) {
        if !self.path.is_empty() {
            self.path += "/";
        }
        self.path += folder_name;
    }

    pub fn as_str(&self) -> &str {
        &self.path
    }
}

impl Into<String> for InternalPath {
    fn into(self) -> String {
        self.path
    }
}
