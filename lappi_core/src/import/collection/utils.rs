use anyhow::Result;

use crate::collection::Collection;
use crate::collection::folders::FolderType;
use crate::collection::music::MusicItemId;
use crate::collection::tags::TagsMap;

pub fn import_song(collection: &Collection, tags: &TagsMap) -> Result<Option<MusicItemId>> {
    let title = match tags.get_string_tag("title") {
        Some(title) => title,
        None => return Ok(None),
    };

    let album = match tags.get_string_tag("album") {
        Some(album) => album,
        None => return Ok(None),
    };

    let artist = match tags.get_string_tag("artist") {
        Some(artist) => artist,
        None => return Ok(None),
    };

    let folders = collection.folders();
    let artist_id = folders.find_or_add_folder(folders.get_root_folder(), artist.clone(), FolderType::Artist)?;
    let album_id = folders.find_or_add_folder(artist_id, album.clone(), FolderType::Album)?;

    let item_id = collection.music().create_item(title.clone(), album_id)?;

    return Ok(Some(item_id));
}

