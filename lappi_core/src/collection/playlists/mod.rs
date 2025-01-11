pub mod database_api;
pub mod types;
pub mod debug;

use std::sync::Arc;

use amina_core::service::{Context, Service, ServiceApi, ServiceInitializer};
use amina_core::rpc::Rpc;
use amina_core::register_rpc_handler;

use crate::database::Database;
use crate::collection::music::{MusicCollection, MusicItemId};

use database_api::PlaylistsDbApi;
use types::{PlaylistDesc, PlaylistId, PlaylistItemDesc};

pub struct PlaylistsCollection {
    db: Box<dyn PlaylistsDbApi>,
    music: Service<MusicCollection>,
}

impl PlaylistsCollection {
    pub fn get_playlists(&self) -> Vec<PlaylistDesc> {
        return self.db.get_playlists().unwrap();
    }

    pub fn get_playlist_description(&self, playlist_id: PlaylistId) -> PlaylistDesc {
        return self.db.get_playlist_description(playlist_id).unwrap();
    }

    pub fn create_playlist(&self, name: String) -> PlaylistId {
        return self.db.create_playlist(&name).unwrap();
    }

    pub fn create_default_playlist(&self) -> PlaylistId {
        let random_name = format!("Playlist {}", rand::random::<u16>());
        return self.db.create_playlist(&random_name).unwrap();
    }

    pub fn set_playlist_name(&self, playlist_id: PlaylistId, name: String) {
        self.db.set_playlist_name(playlist_id, &name).unwrap();
    }

    pub fn delete_playlist(&self, playlist_id: PlaylistId) {
        self.db.delete_playlist(playlist_id).unwrap();
    }

    pub fn get_playlist_items(&self, playlist_id: PlaylistId) -> Vec<PlaylistItemDesc> {
        let playlist_items = self.db.get_playlist_items(playlist_id).unwrap();

        let mut result = Vec::new();

        for (id, music_item_id) in playlist_items {
            let music_item_desc = self.music.get_item_description(music_item_id);
            result.push(PlaylistItemDesc {
                id,
                music_item_id,
                title: music_item_desc.name,
                artist: "".to_string(),
                album: "".to_string(), 
            });
        }

        return result;
    }

    pub fn get_playlists_for_music_item(&self, music_item_id: MusicItemId) -> Vec<PlaylistId> {
        return self.db.get_playlists_for_music_item(music_item_id).unwrap();
    }

    pub fn add_item_to_playlist(&self, playlist_id: PlaylistId, music_item_id: MusicItemId) {
        self.db.add_item_to_playlist(playlist_id, music_item_id).unwrap();
    }

    pub fn delete_item_from_playlist(&self, playlist_id: PlaylistId, music_item_id: MusicItemId) {
        self.db.delete_item_from_playlist(playlist_id, music_item_id).unwrap();
    }
}

impl ServiceApi for PlaylistsCollection {

}

impl ServiceInitializer for PlaylistsCollection {
    fn initialize(context: &Context) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();
        let database = context.get_service::<Database>();

        let playlists = Arc::new(Self {
            db: database.get_playlist(),
            music: context.get_service::<MusicCollection>()
        });

        register_rpc_handler!(rpc, playlists, "lappi.playlists.get_playlists", get_playlists());
        register_rpc_handler!(rpc, playlists, "lappi.playlists.get_playlist_description", get_playlist_description(playlist_id: PlaylistId));
        register_rpc_handler!(rpc, playlists, "lappi.playlists.create_playlist", create_playlist(name: String));
        register_rpc_handler!(rpc, playlists, "lappi.playlists.create_default_playlist", create_default_playlist());
        register_rpc_handler!(rpc, playlists, "lappi.playlists.set_playlist_name", set_playlist_name(playlist_id: PlaylistId, name: String));
        register_rpc_handler!(rpc, playlists, "lappi.playlists.delete_playlist", delete_playlist(playlist_id: PlaylistId));
        register_rpc_handler!(rpc, playlists, "lappi.playlists.get_playlist_items", get_playlist_items(playlist_id: PlaylistId));
        register_rpc_handler!(rpc, playlists, "lappi.playlists.get_playlists_for_music_item", get_playlists_for_music_item(music_item_id: MusicItemId));
        register_rpc_handler!(rpc, playlists, "lappi.playlists.add_item_to_playlist", add_item_to_playlist(playlist_id: PlaylistId, music_item_id: MusicItemId));
        register_rpc_handler!(rpc, playlists, "lappi.playlists.delete_item_from_playlist", delete_item_from_playlist(playlist_id: PlaylistId, music_item_id: MusicItemId));
        
        return playlists;
    }
}
