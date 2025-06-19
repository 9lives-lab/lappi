pub mod types;
pub mod database_api;

use std::sync::Arc;

use anyhow::Result;
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Context, Service, ServiceApi, ServiceInitializer};

use crate::collection::internal_files::{InternalFiles, InternalPath};
use crate::collection::music::MusicCollection;
use crate::database::Database;
use super::music::MusicItemId;

use database_api::LyricsDbApi;

pub use types::*;

#[derive(Clone)]
pub struct LyricsCollection {
    lyrics_db: Arc<Box<dyn LyricsDbApi>>,
    music: Service<MusicCollection>,
    internal_files: Service<InternalFiles>,
}

impl LyricsCollection {
    pub fn add_lyrics_item(&self, music_item_id: MusicItemId, lyrics_tag: String) -> Result<LyricsId> {
        let internal_path = self.gen_internal_path(music_item_id, &lyrics_tag)?;
        let file_id = self.internal_files.add_new_file(&internal_path)?;
        let lyrics_id = self.lyrics_db.add_lyrics_item(music_item_id, &lyrics_tag, file_id)?;
        Ok(lyrics_id)
    }

    pub fn get_lyrics_list(&self, music_id: MusicItemId) -> Result<Vec<LyricsDesc>> {
        self.lyrics_db.get_lyrics_list(music_id)
    }

    pub fn save_lyrics(&self, lyrics_id: LyricsId, text: String) -> Result<()> {
        let file_id = self.lyrics_db.get_lyrics_descriptor(lyrics_id)?.internal_file_id;
        let path = self.internal_files.get_system_path(file_id)?;
        std::fs::write(path, text.as_bytes())?;
        Ok(())
    }

    pub fn get_lyrics(&self, lyrics_id: LyricsId) -> Result<String> {
        let file_id = self.lyrics_db.get_lyrics_descriptor(lyrics_id)?.internal_file_id;
        let path = self.internal_files.get_system_path(file_id)?;
        let file_content = std::fs::read_to_string(path)?;
        Ok(file_content)
    }

    pub fn gen_internal_path(&self, music_item_id: MusicItemId, lyrics_tag: &str) -> Result<InternalPath> {
        let template = "lyrics/{file_name} (".to_string() +  lyrics_tag + ").txt";
        let internal_path = self.music.gen_internal_path(music_item_id, &template)?;
        Ok(internal_path)
    }
}

impl ServiceApi for LyricsCollection {

}

impl ServiceInitializer for LyricsCollection {
    fn initialize(context: &Context) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();
        let database = context.get_service::<Database>();

        let lyrics = Arc::new(Self {
            lyrics_db: Arc::new(database.get_lyrics_api()),
            music: context.get_service::<MusicCollection>(),
            internal_files: context.get_service::<InternalFiles>(),
        });

        register_rpc_handler!(rpc, lyrics, "lappi.collection.lyrics.add_lyrics_item", add_lyrics_item(music_item_id: MusicItemId, lyrics_tag: String));
        register_rpc_handler!(rpc, lyrics, "lappi.collection.lyrics.get_lyrics_list", get_lyrics_list(music_id: MusicItemId));
        register_rpc_handler!(rpc, lyrics, "lappi.collection.lyrics.save_lyrics", save_lyrics(lyrics_id: LyricsId, text: String));
        register_rpc_handler!(rpc, lyrics, "lappi.collection.lyrics.get_lyrics", get_lyrics(lyrics_id: LyricsId));

        return lyrics;
    }
}

