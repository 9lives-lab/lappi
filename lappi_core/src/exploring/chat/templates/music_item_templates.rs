use amina_core::service::{Context, Service};
use crate::collection::Collection;
use super::{ChatTemplate, ChatTemplateContext};

struct LyricsTemplate {
    collection: Service<Collection>,
}

impl LyricsTemplate {
    pub fn create(context: &Context) -> LyricsTemplate {
        return Self {
            collection: context.get_service::<Collection>(),
        };
    }

    pub fn is_applicable(&self, context: &ChatTemplateContext) -> bool {
        match context {
            ChatTemplateContext::Folder(_) => false,
            ChatTemplateContext::MusicItem(_) => true,
        }
    }

    pub fn get_lyrics(&self, context: &ChatTemplateContext) -> String {
        match context {
            ChatTemplateContext::MusicItem(music_item_id) => {
                let lyrics_list = self.collection.lyrics().get_lyrics_list(*music_item_id);
                if lyrics_list.len() > 0 {
                    let desc = lyrics_list.get(0).unwrap();
                    let lyrics = self.collection.lyrics().get_lyrics(desc.lyrics_id);
                    return lyrics;
                } else {
                    return "".to_string();
                }
            },
            ChatTemplateContext::Folder(_) => return "".to_string(),
        }
    }
}

struct TranslateLyrics {
    lyrics_template: LyricsTemplate,
}

impl TranslateLyrics {
    fn create(context: &Context) -> Box<dyn ChatTemplate> {
        return Box::new(Self {
            lyrics_template: LyricsTemplate::create(context),
        })
    }
}

impl ChatTemplate for TranslateLyrics {
    fn get_name(&self) -> &str {
        return "Translate lyrics"
    }

    fn is_applicable(&self, context: &ChatTemplateContext) -> bool {
        return self.lyrics_template.is_applicable(context);
    }

    fn get_message(&self, context: &ChatTemplateContext) -> String {
        let lyrics = self.lyrics_template.get_lyrics(context);
        return format!("Could you translate the following lyrics to English? \n\n {}", lyrics);
    }
}

struct ExplainLyrics {
    lyrics_template: LyricsTemplate,
}

impl ExplainLyrics {
    fn create(context: &Context) -> Box<dyn ChatTemplate> {
        return Box::new(Self {
            lyrics_template: LyricsTemplate::create(context),
        })
    }
}

impl ChatTemplate for ExplainLyrics {
    fn get_name(&self) -> &str {
        return "Explain lyrics"
    }

    fn is_applicable(&self, context: &ChatTemplateContext) -> bool {
        return self.lyrics_template.is_applicable(context);
    }

    fn get_message(&self, context: &ChatTemplateContext) -> String {
        let lyrics = self.lyrics_template.get_lyrics(context);
        return format!("Could you explain the following lyrics? \n\n {}", lyrics);
    }
}

pub fn get_templates(context: &Context) -> Vec<Box<dyn ChatTemplate>> {
    let mut templates = Vec::new();
    templates.push(TranslateLyrics::create(context));
    templates.push(ExplainLyrics::create(context));
    return templates;
}
