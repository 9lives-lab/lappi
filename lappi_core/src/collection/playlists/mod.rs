pub mod database_api;
pub mod types;

use std::sync::Arc;

use anyhow::Result;
use amina_core::service::{AppContext, Service, ServiceApi, ServiceInitializer};
use amina_core::rpc::Rpc;
use amina_core::register_rpc_handler;

use crate::database::Database;
use crate::collection::music::{MusicCollection, MusicItemId};
use crate::collection::pictures::PictureId;

use database_api::PlaylistsDbApi;
use types::{PlaylistDesc, PlaylistId, PlaylistItemDesc};

pub struct PlaylistsCollection {
    db: Box<dyn PlaylistsDbApi>,
    music: Service<MusicCollection>,
}

impl PlaylistsCollection {
    pub fn get_playlists(&self) -> Result<Vec<PlaylistDesc>> {
        self.db.get_playlists()
    }

    pub fn get_playlist_description(&self, playlist_id: PlaylistId) -> Result<PlaylistDesc> {
        self.db.get_playlist_description(playlist_id)
    }

    pub fn create_playlist(&self, name: String) -> Result<PlaylistId> {
        self.db.create_playlist(&name)
    }

    pub fn create_default_playlist(&self) -> Result<PlaylistId> {
        let random_name = format!("Playlist {}", rand::random::<u16>());
        self.db.create_playlist(&random_name)
    }

    pub fn set_playlist_name(&self, playlist_id: PlaylistId, name: String) -> Result<()> {
        self.db.set_playlist_name(playlist_id, &name)
    }

    pub fn set_playlist_cover(&self, playlist_id: PlaylistId, picture_id: PictureId) -> Result<()> {
        log::info!("Set playlist cover. Playlist id: {} -> Picture id:  {}", playlist_id, picture_id);
        self.db.set_playlist_cover(playlist_id, Some(picture_id))
    }

    pub fn remove_playlist_cover(&self, playlist_id: PlaylistId) -> Result<()> {
        log::info!("Remove playlist cover. Playlist id: {}", playlist_id);
        self.db.set_playlist_cover(playlist_id, None)
    }

    pub fn delete_playlist(&self, playlist_id: PlaylistId) -> Result<()> {
        self.db.delete_playlist(playlist_id)
    }

    pub fn get_playlist_items(&self, playlist_id: PlaylistId) -> Result<Vec<PlaylistItemDesc>> {
        let playlist_items = self.db.get_playlist_items(playlist_id)?;

        let mut result = Vec::new();

        for (id, music_item_id) in playlist_items {
            let music_item_desc = self.music.get_item_description(music_item_id)?;
            let artist = self.music.get_tag(music_item_id, "artist")?
                .map(|tag| tag.to_string())
                .unwrap_or_else(|| "".to_string());
            let album = self.music.get_tag(music_item_id, "album")?
                .map(|tag| tag.to_string())
                .unwrap_or_else(|| "".to_string());

            result.push(PlaylistItemDesc {
                id,
                music_item_id,
                title: music_item_desc.name,
                artist,
                album, 
            });
        }

        Ok(result)
    }

    pub fn get_playlists_for_music_item(&self, music_item_id: MusicItemId) -> Result<Vec<PlaylistId>> {
        return self.db.get_playlists_for_music_item(music_item_id)
    }

    pub fn add_item_to_playlist(&self, playlist_id: PlaylistId, music_item_id: MusicItemId) -> Result<()> {
        self.db.add_item_to_playlist(playlist_id, music_item_id)
    }

    pub fn delete_item_from_playlist(&self, playlist_id: PlaylistId, music_item_id: MusicItemId) -> Result<()> {
        self.db.delete_item_from_playlist(playlist_id, music_item_id)
    }
}

impl ServiceApi for PlaylistsCollection {

}

impl ServiceInitializer for PlaylistsCollection {
    fn initialize(context: &AppContext) -> Arc<Self> {
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
        register_rpc_handler!(rpc, playlists, "lappi.playlists.set_playlist_cover", set_playlist_cover(playlist_id: PlaylistId, picture_id: PictureId));
        register_rpc_handler!(rpc, playlists, "lappi.playlists.remove_playlist_cover", remove_playlist_cover(playlist_id: PlaylistId));
        register_rpc_handler!(rpc, playlists, "lappi.playlists.delete_playlist", delete_playlist(playlist_id: PlaylistId));
        register_rpc_handler!(rpc, playlists, "lappi.playlists.get_playlist_items", get_playlist_items(playlist_id: PlaylistId));
        register_rpc_handler!(rpc, playlists, "lappi.playlists.get_playlists_for_music_item", get_playlists_for_music_item(music_item_id: MusicItemId));
        register_rpc_handler!(rpc, playlists, "lappi.playlists.add_item_to_playlist", add_item_to_playlist(playlist_id: PlaylistId, music_item_id: MusicItemId));
        register_rpc_handler!(rpc, playlists, "lappi.playlists.delete_item_from_playlist", delete_item_from_playlist(playlist_id: PlaylistId, music_item_id: MusicItemId));
        
        return playlists;
    }
}
