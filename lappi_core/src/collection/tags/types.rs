use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub enum TagValue {
    String(String),
    Number(i32),
    Bool,
}

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Tag {
    key: String,
    value: TagValue,
}

impl Tag {
    pub fn new(key: String, value: TagValue) -> Tag {
        Tag { key, value }
    }

    pub fn new_string(key: String, value: String) -> Tag {
        Tag {
            key,
            value: TagValue::String(value),
        }
    }

    pub fn new_number(key: String, value: i32) -> Tag {
        Tag {
            key,
            value: TagValue::Number(value),
        }
    }

    pub fn get_key(&self) -> &str {
        return self.key.as_str();
    }

    pub fn get_value(&self) -> &TagValue {
        return &self.value;
    }

    pub fn to_string(&self) -> String {
        match &self.value {
            TagValue::String(string_value) => string_value.clone(),
            TagValue::Number(number_value) => number_value.to_string(),
            TagValue::Bool => "true".to_string()
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TagsMap {
    map: HashMap<String, TagValue>,
}

impl TagsMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn from_map(map: HashMap<String, String>) -> Self {
        Self {
            map: map.into_iter().map(|(key, value)| (key, TagValue::String(value))).collect()
        }
    }

    pub fn add_tag(&mut self,key: &str, tag_value: TagValue) {
        self.map.insert(key.to_string(), tag_value);
    }

    pub fn add_string_tag(&mut self, key: &str, tag_value: String) {
        self.map.insert(key.to_string(), TagValue::String(tag_value));
    }

    pub fn get_tag(&self, key: &str) -> Option<&TagValue> {
        self.map.get(key)
    }

    pub fn get_string_tag(&self, key: &str) -> Option<&String> {
        match self.map.get(key) {
            Some(value) => match value {
                TagValue::String(text) => Some(text),
                _ => None
            },
            None => None
        }
    }

    pub fn get_tags_map(&self) -> &HashMap<String, TagValue> {
        return &self.map;
    }
}

