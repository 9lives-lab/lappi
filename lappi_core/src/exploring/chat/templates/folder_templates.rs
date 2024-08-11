use amina_core::service::{Context, Service};
use crate::collection::{folders::FolderType, Collection};
use super::{ChatTemplate, ChatTemplateContext};

struct ArtistTemplate {
    collection: Service<Collection>,
}

impl ArtistTemplate {
    pub fn create(context: &Context) -> ArtistTemplate {
        return Self {
            collection: context.get_service::<Collection>(),
        };
    }

    pub fn is_applicable(&self, context: &ChatTemplateContext) -> bool {
        match context {
            ChatTemplateContext::Folder(folder_id) => {
                let desc = self.collection.folders().get_folder_description(*folder_id);
                return desc.folder_type == FolderType::Artist; 
            },
            ChatTemplateContext::MusicItem(_) => false,
        }
    }

    fn get_artist_name(&self, context: &ChatTemplateContext) -> String {
        match context {
            ChatTemplateContext::Folder(folder_id) => {
                let desc = self.collection.folders().get_folder_description(*folder_id);
                return desc.name;
            },
            ChatTemplateContext::MusicItem(_) => panic!("Unexpected template use"),
        }
    }
}

struct ArtistHistory {
    artist_template: ArtistTemplate
}

impl ArtistHistory {
    fn create(context: &Context) -> Box<dyn ChatTemplate> {
        return Box::new(Self {
            artist_template: ArtistTemplate::create(context),
        });
    }
}

impl ChatTemplate for ArtistHistory {
    fn get_name(&self) -> &str {
        return "Tell artist history"
    }

    fn is_applicable(&self, context: &ChatTemplateContext) -> bool {
        return self.artist_template.is_applicable(context);
    }

    fn get_message(&self, context: &ChatTemplateContext) -> String {
        let artist_name = self.artist_template.get_artist_name(context);
        return format!("Could you tell the history of {} music artist?\nPlease tell in 15-20 sentences.", artist_name);
    }
}

struct SimilarArtist {
    artist_template: ArtistTemplate
}

impl SimilarArtist {
    fn create(context: &Context) -> Box<dyn ChatTemplate> {
        return Box::new(Self {
            artist_template: ArtistTemplate::create(context),
        });
    }
}

impl ChatTemplate for SimilarArtist {
    fn get_name(&self) -> &str {
        return "Recommend similar"
    }

    fn is_applicable(&self, context: &ChatTemplateContext) -> bool {
        return self.artist_template.is_applicable(context);
    }

    fn get_message(&self, context: &ChatTemplateContext) -> String {
        let artist_name = self.artist_template.get_artist_name(context);
        return format!("Please recommend artists whose music is similar to {}", artist_name);
    }
}

pub fn get_templates(context: &Context) -> Vec<Box<dyn ChatTemplate>> {
    let mut templates = Vec::new();
    templates.push(ArtistHistory::create(context));
    templates.push(SimilarArtist::create(context));
    return templates;
}
