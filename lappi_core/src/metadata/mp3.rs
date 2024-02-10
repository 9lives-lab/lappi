use std::io::Read;

use id3::Tag;
use id3::TagLike;
use crate::collection::types::tags::{TagsMap, TagValue};
use crate::metadata::Metadata;

pub fn read(reader: Box<dyn Read>) -> Metadata {
    let mut tags = TagsMap::new();

    match Tag::read_from(reader) {
        Ok(id3_tags) => {
            add_string_tag(&mut tags, "title", id3_tags.title());
            add_string_tag(&mut tags, "album", id3_tags.album());
            add_string_tag(&mut tags, "artist", id3_tags.artist());
            //add_int_tag(&mut tags, "year", id3_tags.year());
        },
        Err(_) => {}
    }

    return Metadata {
        media_type: String::from("audio"),
        tags
    }
}

fn add_string_tag(tags: &mut TagsMap, name: &str, value: Option<&str>) {
    if let Some(value) = value {
        tags.add_tag(name, TagValue::String(String::from(value)));
    }
}

