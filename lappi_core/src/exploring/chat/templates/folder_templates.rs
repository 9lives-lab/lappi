use anyhow::Result;
use amina_core::service::{AppContext, Service};

use crate::collection::{folders::FolderType, Collection};
use super::{ChatTemplate, ChatTemplateContext};

struct ArtistTemplate {
    collection: Service<Collection>,
}

impl ArtistTemplate {
    pub fn create(context: &AppContext) -> ArtistTemplate {
        Self {
            collection: context.get_service::<Collection>(),
        }
    }

    pub fn is_applicable(&self, context: &ChatTemplateContext) -> Result<bool> {
        Ok(match context {
            ChatTemplateContext::Folder(folder_id) => {
                let desc = self.collection.folders().get_folder_description(*folder_id)?;
                desc.folder_type == FolderType::Artist 
            },
            ChatTemplateContext::MusicItem(_) => false,
        })
    }

    fn get_artist_name(&self, context: &ChatTemplateContext) -> Result<String> {
        match context {
            ChatTemplateContext::Folder(folder_id) => {
                let desc = self.collection.folders().get_folder_description(*folder_id)?;
                Ok(desc.name)
            },
            _ => Err(anyhow::anyhow!("ArtistTemplate is not applicable for {:?}", context)),
        }
    }
}

struct ArtistHistory {
    artist_template: ArtistTemplate
}

impl ArtistHistory {
    fn create(context: &AppContext) -> Box<dyn ChatTemplate> {
        Box::new(Self {
            artist_template: ArtistTemplate::create(context),
        })
    }
}

impl ChatTemplate for ArtistHistory {
    fn get_name(&self) -> &str {
        "Tell artist history"
    }

    fn is_applicable(&self, context: &ChatTemplateContext) -> Result<bool> {
        self.artist_template.is_applicable(context)
    }

    fn get_message(&self, context: &ChatTemplateContext) -> Result<String> {
        let artist_name = self.artist_template.get_artist_name(context)?;
        Ok(format!("Tell the history of {} music artist in 15-20 sentences", artist_name))
    }
}

struct SimilarArtist {
    artist_template: ArtistTemplate
}

impl SimilarArtist {
    fn create(context: &AppContext) -> Box<dyn ChatTemplate> {
        Box::new(Self {
            artist_template: ArtistTemplate::create(context),
        })
    }
}

impl ChatTemplate for SimilarArtist {
    fn get_name(&self) -> &str {
        "Recommend similar"
    }

    fn is_applicable(&self, context: &ChatTemplateContext) -> Result<bool> {
        self.artist_template.is_applicable(context)
    }

    fn get_message(&self, context: &ChatTemplateContext) -> Result<String> {
        let artist_name = self.artist_template.get_artist_name(context)?;
        Ok(format!("Recommend artists whose music is similar to {}", artist_name))
    }
}

pub fn get_templates(context: &AppContext) -> Vec<Box<dyn ChatTemplate>> {
    let mut templates = Vec::new();
    templates.push(ArtistHistory::create(context));
    templates.push(SimilarArtist::create(context));
    return templates;
}
