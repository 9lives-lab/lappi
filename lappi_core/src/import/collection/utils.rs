use crate::collection::Collection;
use crate::collection::folders::FolderType;
use crate::collection::music::MusicItemId;
use crate::collection::tags::TagsMap;

pub fn import_song(collection: &Collection, tags: &TagsMap) -> Option<MusicItemId> {
    let title = tags.get_string_tag("title");
    let album = tags.get_string_tag("album");
    let artist = tags.get_string_tag("artist");
    if title.is_none() || album.is_none() || artist.is_none() {
        return None;
    }

    let folders = collection.folders();
    let artist_id = folders.find_or_add_folder(folders.get_root_folder(), artist.unwrap().clone(), FolderType::Artist);
    let album_id = folders.find_or_add_folder(artist_id, album.unwrap().clone(), FolderType::Album);

    let item_id = collection.music().create_item(title.unwrap().clone(), album_id);

    collection.music().set_tag(item_id, "title".to_string(), title.unwrap().to_string());
    collection.music().set_tag(item_id, "album".to_string(), album.unwrap().to_string());

    return Some(item_id);
}

