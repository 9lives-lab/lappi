use std::fs::File;
use std::path::{Path, PathBuf};

use amina_core::service::Service;
use serde::Deserialize;

use crate::collection::folders::{FolderId, FolderType};
use crate::collection::Collection;

#[derive(Debug, Deserialize)]
struct CollectionEntry {
    playlists: Option<Vec<String>>,
    artists: Vec<ArtistEntry>,
}

#[derive(Debug, Deserialize)]
struct ArtistEntry {
    name: String,
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
}

pub struct BasicYamlCollectionImporter {
    collection: Service<Collection>,
    dir_path: PathBuf,
}

impl BasicYamlCollectionImporter {

    pub fn new(collection: Service<Collection>, dir_path: &Path) -> Self {
        Self {
            collection,
            dir_path: dir_path.to_path_buf(),
        }
    }

    pub fn import(&self) {
        let mut file_path = self.dir_path.clone();
        file_path.push("collection.yaml");
        let file = File::open(file_path).unwrap();
        let collection_entry: CollectionEntry = serde_yaml::from_reader(file).unwrap();
        let collection = &self.collection;
        collection.start_batch();
        self.import_playlists(collection_entry.playlists);
        self.import_artists(collection_entry.artists);
        collection.stop_batch();
    }

    fn import_playlists(&self, playlists: Option<Vec<String>>) {
        for playlist in playlists.unwrap_or_default() {
            self.collection.playlists().create_playlist(playlist);
        }
    }

    fn import_artists(&self, artists: Vec<ArtistEntry>) {
        for artist_entry in artists {
            let artist_name = artist_entry.name;
            let root_folder = self.collection.folders().get_root_folder();
            let artist_folder = self.collection.folders().find_or_add_folder(root_folder, artist_name, FolderType::Artist);
            self.import_pictures(artist_entry.pictures, artist_folder);
            let albums = artist_entry.albums.unwrap_or_default();
            self.import_albumns(albums, artist_folder);
        }
    }

    fn import_albumns(&self, albums: Vec<AlbumEntry>, parent_folder_id: FolderId) {
        for album_entry in albums {
            let album_name = album_entry.name;
            let album_folder = self.collection.folders().find_or_add_folder(parent_folder_id, album_name, FolderType::Album);
            self.import_pictures(album_entry.pictures, album_folder);
            let songs = album_entry.songs.unwrap_or_default();
            self.import_songs(songs, album_folder);
            if let Some(year) = album_entry.year {
                let year_tag = year.to_string();
                self.collection.folders().set_tag(album_folder, "year".to_string(), year_tag);
            }
        }
    }

    fn import_songs(&self, songs: Vec<SongEntry>, parent_folder_id: FolderId) {
        for song_entry in songs {
            self.collection.music().create_item(song_entry.name, parent_folder_id);
        }
    }

    fn import_pictures(&self, pictures: Option<Vec<String>>, folder_id: FolderId) {
        let mut first = true;
        for picture_entry in pictures.unwrap_or_default() {
            let mut file_path = self.dir_path.clone();
            file_path.push(picture_entry);
            let picture_id = self.collection.pictures().copy_to_collection_by_path(file_path.to_str().unwrap().to_string(), folder_id);
            if first {
                self.collection.folders().set_folder_cover(folder_id, picture_id);
                first = false;
            }
        }
    }

}

