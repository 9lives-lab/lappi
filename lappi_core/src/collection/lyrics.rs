use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Context, Service};

use crate::collection::database_api::DatabaseApi;
use crate::collection::storage::local::LocalStorage;
use crate::collection::types::{LyricsId, MusicItemId};
use crate::database::Database;

#[derive(Serialize, Deserialize)]
pub struct LyricsDescription {
    pub lyrics_id: LyricsId,
    pub lang_code: String,
}

#[derive(Clone)]
pub struct LyricsCollection {
    db: Arc<Box<dyn DatabaseApi>>,
    local_storage: Service<LocalStorage>,
}

impl LyricsCollection {
    pub fn initialize(context: &Context) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();
        let database = context.get_service::<Database>();
        let db_api = Arc::new(database.collection());
        let local_storage = context.get_service::<LocalStorage>();

        let lyrics = Arc::new(Self {
            db: db_api,
            local_storage,
        });

        register_rpc_handler!(rpc, lyrics, "lappi.collection.lyrics.add_lyrics_item", add_lyrics_item(music_item_id: MusicItemId, lang_code: String));
        register_rpc_handler!(rpc, lyrics, "lappi.collection.lyrics.get_lyrics_list", get_lyrics_list(music_id: MusicItemId));
        register_rpc_handler!(rpc, lyrics, "lappi.collection.lyrics.save_lyrics", save_lyrics(lyrics_id: LyricsId, text: String));
        register_rpc_handler!(rpc, lyrics, "lappi.collection.lyrics.get_lyrics", get_lyrics(lyrics_id: LyricsId));

        return lyrics;
    }

    pub fn add_lyrics_item(&self, music_item_id: MusicItemId, lang_code: String) -> LyricsId {
        let lyrics_id = self.db.add_lyrics_item(music_item_id, &lang_code).unwrap();
        self.save_lyrics(lyrics_id, "".to_string());
        return lyrics_id;
    }

    pub fn get_lyrics_list(&self, music_id: MusicItemId) -> Vec<LyricsDescription> {
        self.db.get_lyrics_list(music_id).unwrap()
    }

    pub fn save_lyrics(&self, lyrics_id: LyricsId, text: String) {
        let path = self.get_lyrics_storage_path(lyrics_id);
        std::fs::write(path, text.as_bytes()).unwrap();
    }

    pub fn get_lyrics(&self, lyrics_id: LyricsId) -> String {
        let path = self.get_lyrics_storage_path(lyrics_id);
        let file_content = std::fs::read_to_string(path).unwrap();
        return file_content;
    }

    fn get_lyrics_storage_path(&self, lyrics_id: LyricsId) -> PathBuf {
        return self.local_storage.get_internal_storage_folder("lyrics").join(format!("{}.txt", lyrics_id));
    }
}

