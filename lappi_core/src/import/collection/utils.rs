use crate::collection::Collection;
use crate::collection::types::{ItemId};
use crate::collection::types::tags::TagsMap;

pub fn import_song(collection: &Collection, tags: &TagsMap) -> Option<ItemId> {
    let item_id = collection.create_item();

    let title = tags.get_string_tag("title");
    let album = tags.get_string_tag("album");
    let artist = tags.get_string_tag("artist");
    if title.is_none() || album.is_none() || artist.is_none() {
        return None;
    }

    let artist_id = collection.artists().find_by_name(artist.unwrap().clone());
    collection.music().add_artist(item_id, artist_id);

    collection.add_tag(item_id, "title", title.unwrap());
    collection.add_tag(item_id, "album", album.unwrap());

    return Some(item_id);
}

