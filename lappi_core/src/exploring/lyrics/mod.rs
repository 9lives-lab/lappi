pub mod sources;

use std::sync::Arc;
use amina_core::{register_rpc_handler, rpc::Rpc};
use amina_core::service::{Context, Service, ServiceApi, ServiceInitializer};

use sources::{lyrics_ovh::LyricsOvhSource, LyricsSource};
use crate::collection::folders::FolderType;
use crate::collection::music::MusicItemId;
use crate::collection::Collection;

use super::{ExploringError, ExploringResult, SourceList};


pub struct LyricsExplorer {
    collection: Service<Collection>,
    sources: SourceList<LyricsSource>,
}

impl LyricsExplorer {
    pub fn get_source_list(&self) -> Vec<String> {
        self.sources.get_source_list()
    }

    pub fn find_lyrics(&self, music_item_id: MusicItemId) -> ExploringResult<String> {
        let description = self.collection.music().get_item_description(music_item_id);

        let title = description.name;
        let artist = match self.collection.folders().find_parent_node(description.folder_id, FolderType::Artist) {
            Some(folder_description) => folder_description.name,
            None => return Err(ExploringError::GenericError("No artist folder found".to_string()))
        };

        log::info!("Search lyrics for item_id: {}, artist: {}, song: {}", music_item_id, &artist, &title);

        for source in self.sources.get_sources() {
            match source.find_lyrics(&artist, &title) {
                Ok(mut lyrics_text) => {
                    log::info!("Lyrics found on {}", source.source_name());
                    let footer = format!("\n\n[Lyrics from {}]", source.source_name());
                    lyrics_text = lyrics_text + &footer;
                    return Ok(lyrics_text)
                },
                Err(err) => {
                    log::debug!("{} error: {:?}", source.source_name(), err);
                },
            }
        }

        log::error!("No lyrics found");

        return Err(ExploringError::GenericError("No lyrics found".to_string()));
    }
}

impl ServiceApi for LyricsExplorer {

}

impl ServiceInitializer for LyricsExplorer {
    fn initialize(context: &Context) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();
        let collection = context.get_service::<Collection>();

        let sources = SourceList::new();
        sources.add_source(LyricsSource::new(LyricsOvhSource::new()));

        let lyrics_explorer = Arc::new(LyricsExplorer {
            collection,
            sources,
        });

        register_rpc_handler!(rpc, lyrics_explorer, "lappi.exploring.lyrics.get_source_list", get_source_list());
        register_rpc_handler!(rpc, lyrics_explorer, "lappi.exploring.lyrics.find_lyrics", find_lyrics(music_item_id: MusicItemId));

        return lyrics_explorer;
    }
}
