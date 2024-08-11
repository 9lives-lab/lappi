mod folder_templates;
mod music_item_templates;

use std::sync::Arc;
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Context, Service, ServiceApi, ServiceInitializer};
use crate::collection::types::{FolderId, MusicItemId};
use super::ChatService;

pub type TemplateId = i32;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TemplateDesc {
    id: TemplateId,
    name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ChatTemplateContext {
    Folder(FolderId),
    MusicItem(MusicItemId),
}

pub trait ChatTemplate: Send + Sync {
    fn get_name(&self) -> &str;
    fn is_applicable(&self, context: &ChatTemplateContext) -> bool;
    fn get_message(&self, context: &ChatTemplateContext) -> String;
}

pub struct ChatTemplates {
    chat_service: Service<ChatService>,
    templates: Vec<Box<dyn ChatTemplate>>,
}

impl ChatTemplates {
    pub fn get_templates_list(&self, context: ChatTemplateContext) -> Vec<TemplateDesc> {
        self.templates.iter()
        .enumerate()
        .filter_map(|(i, template)| {
            if template.is_applicable(&context) {
                return Some(TemplateDesc {
                    id: i as TemplateId,
                    name: template.get_name().to_string(),
                })
            } else {
                return None;
            }
        })
        .collect()
    }

    pub fn get_message(&self, id: TemplateId, context: ChatTemplateContext) -> String {
        let template = self.templates.get(id as usize).unwrap();
        return template.get_message(&context);
    }

    pub fn create_chat_from_template(&self, id: TemplateId, context: ChatTemplateContext) {
        let message = self.get_message(id, context);
        self.chat_service.create_chat();
        self.chat_service.send_message(message);
    }
}

impl ServiceApi for ChatTemplates {

}

impl ServiceInitializer for ChatTemplates {
    fn initialize(context: &Context) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();
        let chat_service = context.get_service::<ChatService>();

        let mut templates = Vec::new();
        templates.append(&mut folder_templates::get_templates(context));
        templates.append(&mut music_item_templates::get_templates(context));

        let chat_templates = Arc::new(Self {
            chat_service,
            templates,
        });

        register_rpc_handler!(rpc, chat_templates, "lappi.chat.templates.get_templates_list", get_templates_list(context: ChatTemplateContext));
        register_rpc_handler!(rpc, chat_templates, "lappi.chat.templates.get_message", get_message(id: TemplateId, context: ChatTemplateContext));
        register_rpc_handler!(rpc, chat_templates, "lappi.chat.templates.create_chat_from_template", create_chat_from_template(id: TemplateId, context: ChatTemplateContext));

        return chat_templates;
    }
}

