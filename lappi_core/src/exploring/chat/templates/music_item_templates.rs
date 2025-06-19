use anyhow::{Context, Result};
use amina_core::service::{AppContext, Service};

use crate::collection::Collection;
use super::{ChatTemplate, ChatTemplateContext};

struct LyricsTemplate {
    collection: Service<Collection>,
}

impl LyricsTemplate {
    pub fn create(context: &AppContext) -> LyricsTemplate {
        Self {
            collection: context.get_service::<Collection>(),
        }
    }

    pub fn is_applicable(&self, context: &ChatTemplateContext) -> Result<bool> {
        Ok(self.get_lyrics(context)?.is_some())
    }

    pub fn get_lyrics(&self, context: &ChatTemplateContext) -> Result<Option<String>> {
        Ok(match context {
            ChatTemplateContext::MusicItem(music_item_id) => {
                let lyrics_list = self.collection.lyrics().get_lyrics_list(*music_item_id)?;
                if let Some(desc) = lyrics_list.get(0) {
                    let lyrics = self.collection.lyrics().get_lyrics(desc.lyrics_id)?;
                    Some(lyrics)
                } else {
                    None
                }
            },
            ChatTemplateContext::Folder(_) => None,
        })
    }
}

struct TranslateLyrics {
    lyrics_template: LyricsTemplate,
}

impl TranslateLyrics {
    fn create(context: &AppContext) -> Box<dyn ChatTemplate> {
        Box::new(Self {
            lyrics_template: LyricsTemplate::create(context),
        })
    }
}

impl ChatTemplate for TranslateLyrics {
    fn get_name(&self) -> &str {
        "Translate lyrics"
    }

    fn is_applicable(&self, context: &ChatTemplateContext) -> Result<bool> {
        self.lyrics_template.is_applicable(context)
    }

    fn get_message(&self, context: &ChatTemplateContext) -> Result<String> {
        let lyrics = self.lyrics_template.get_lyrics(context)?.context("No lyrics found")?;
        Ok(format!("Translate the following lyrics to English \n\n {}", lyrics))
    }
}

struct ExplainLyrics {
    lyrics_template: LyricsTemplate,
}

impl ExplainLyrics {
    fn create(context: &AppContext) -> Box<dyn ChatTemplate> {
        Box::new(Self {
            lyrics_template: LyricsTemplate::create(context),
        })
    }
}

impl ChatTemplate for ExplainLyrics {
    fn get_name(&self) -> &str {
        "Explain lyrics"
    }

    fn is_applicable(&self, context: &ChatTemplateContext) -> Result<bool> {
        self.lyrics_template.is_applicable(context)
    }

    fn get_message(&self, context: &ChatTemplateContext) -> Result<String> {
        let lyrics = self.lyrics_template.get_lyrics(context)?.context("No lyrics found")?;
        Ok(format!("Explain the following lyrics \n\n {}", lyrics))
    }
}

pub fn get_templates(context: &AppContext) -> Vec<Box<dyn ChatTemplate>> {
    let mut templates = Vec::new();
    templates.push(TranslateLyrics::create(context));
    templates.push(ExplainLyrics::create(context));
    return templates;
}
