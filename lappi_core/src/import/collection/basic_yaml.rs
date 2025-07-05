use std::fs::File;
use std::io::Read;

use anyhow::Result;
use camino::Utf8PathBuf;
use serde::Deserialize;
use amina_core::service::Service;

use crate::collection::folders::{FolderId, FolderType};
use crate::collection::tags::TagValue;
use crate::collection::Collection;

#[derive(Debug, Deserialize)]
struct CollectionEntry {
    playlists: Option<Vec<String>>,
    artists: Vec<ArtistEntry>,
}

#[derive(Debug, Deserialize)]
struct ArtistEntry {
    name: String,
    #[serde(default)]
    create_playlist: bool,
    pictures: Option<Vec<String>>,
    albums: Option<Vec<AlbumEntry>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AlbumEntry {
    name: String,
    year: Option<i32>,
    pictures: Option<Vec<String>>,
    songs: Option<Vec<SongEntry>>,
}

#[derive(Debug, Deserialize)]
struct SongEntry {
    name: String,
    file: Option<String>,
    lyrics_file: Option<String>
}

pub struct BasicYamlCollectionImporter {
    collection: Service<Collection>,
    dir_path: Utf8PathBuf,
}

impl BasicYamlCollectionImporter {

    pub fn new(dir_path: Utf8PathBuf) -> Self {
        Self {
            collection: crate::context().get_service::<Collection>(),
            dir_path,
        }
    }

    pub fn import(&self) -> Result<()> {
        let mut file_path = self.dir_path.clone();
        file_path.push("collection.yaml");
        let file = File::open(file_path)?;
        let collection_entry: CollectionEntry = serde_yaml::from_reader(file)?;
        let collection = &self.collection;
    
        collection.start_batch();
        self.import_playlists(collection_entry.playlists)?;
        self.import_artists(collection_entry.artists)?;
        collection.stop_batch();
        Ok(())
    }

    fn import_playlists(&self, playlists: Option<Vec<String>>) -> Result<()> {
        for playlist in playlists.unwrap_or_default() {
            self.collection.playlists().create_playlist(playlist)?;
        }
        Ok(())
    }

    fn import_artists(&self, artists: Vec<ArtistEntry>) -> Result<()> {
        for artist_entry in artists {
            let artist_name = artist_entry.name;

            let root_folder = self.collection.folders().get_root_folder();
            let artist_folder = self.collection.folders().find_or_add_folder(root_folder, artist_name.clone(), FolderType::Artist)?;
            self.import_pictures(artist_entry.pictures, artist_folder)?;

            let albums = artist_entry.albums.unwrap_or_default();
            self.import_albumns(albums, artist_folder)?;

            if artist_entry.create_playlist {
                let playlist_id = self.collection.playlists().create_playlist(artist_name.clone())?;
                let avatar_picture_id = self.collection.folders().get_folder_description(artist_folder)?.avatar_picture_id;
                if let Some(avatar_picture_id) = avatar_picture_id {
                    self.collection.playlists().set_playlist_cover(playlist_id, avatar_picture_id)?;
                }
            }
        }
        Ok(())
    }

    fn import_albumns(&self, albums: Vec<AlbumEntry>, parent_folder_id: FolderId) -> Result<()> {
        for album_entry in albums {
            let album_name = album_entry.name;
            let album_folder = self.collection.folders().find_or_add_folder(parent_folder_id, album_name, FolderType::Album)?;

            if let Some(year) = album_entry.year {
                self.collection.folders().set_tag(album_folder, "year".to_string(), TagValue::Number(year))?;
            }

            self.import_pictures(album_entry.pictures, album_folder)?;

            let songs = album_entry.songs.unwrap_or_default();
            self.import_songs(songs, album_folder)?;
        }
        Ok(())
    }

    fn import_songs(&self, songs: Vec<SongEntry>, parent_folder_id: FolderId) -> Result<()> {
        for (i, song_entry) in songs.iter().enumerate() {
            let music_item_id = self.collection.music().create_item(song_entry.name.clone(), parent_folder_id)?;
            self.collection.music().set_tag(music_item_id, "track".to_string(), TagValue::Number(i as i32 + 1))?;

            if let Some(file) = &song_entry.file {
                log::debug!("Adding file {} to item {}", file, music_item_id);
                let file_path = self.dir_path.clone().join(file).canonicalize_utf8()?;
                self.collection.music_sources().import_music_file(music_item_id, &file_path)?;
            }

            if let Some(lyrics_file) = &song_entry.lyrics_file {
                let file_path = self.dir_path.clone().join(lyrics_file).canonicalize()?.to_string_lossy().to_string();
                let mut lyrics_txt = String::new();
                log::debug!("Adding lyrics file {} to item {}", file_path, music_item_id);
                let mut file = File::open(file_path)?;
                file.read_to_string(&mut lyrics_txt)?;
                let lyrics_tag = "original".to_string();
                let lyrics_id = self.collection.lyrics().add_lyrics_item(music_item_id, lyrics_tag)?;
                self.collection.lyrics().save_lyrics(lyrics_id, lyrics_txt)?;
            }
        }
        Ok(())
    }

    fn import_pictures(&self, pictures: Option<Vec<String>>, folder_id: FolderId) -> Result<()> {
        let mut first = true;
        for picture_entry in pictures.unwrap_or_default() {
            let mut file_path = self.dir_path.clone();
            file_path.push(picture_entry);
            let file_path = file_path.to_string();
            let picture_id = self.collection.pictures().copy_to_collection_by_path(file_path, folder_id)?;
            if first {
                self.collection.folders().set_folder_cover(folder_id, picture_id)?;
                first = false;
            }
        }
        Ok(())
    }

}

