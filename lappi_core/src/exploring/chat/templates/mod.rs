mod folder_templates;
mod music_item_templates;

use std::sync::Arc;

use anyhow::{Context, Result};
use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{AppContext, Service, ServiceApi, ServiceInitializer};

use crate::collection::folders::FolderId;
use crate::collection::music::MusicItemId;

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
    fn is_applicable(&self, context: &ChatTemplateContext) -> Result<bool>;
    fn get_message(&self, context: &ChatTemplateContext) -> Result<String>;
}

pub struct ChatTemplates {
    chat_service: Service<ChatService>,
    templates: Vec<Box<dyn ChatTemplate>>,
}

impl ChatTemplates {
    pub fn get_templates_list(&self, context: ChatTemplateContext) -> Result<Vec<TemplateDesc>> {
        let mut result = Vec::new();

        for (i, template) in self.templates.iter().enumerate() {
            if template.is_applicable(&context)? {
                result.push(TemplateDesc {
                    id: i as TemplateId,
                    name: template.get_name().to_string(),
                });
            }
        }

        Ok(result)
    }

    pub fn get_message(&self, id: TemplateId, context: ChatTemplateContext) -> Result<String> {
        let template = self.templates.get(id as usize).context("Template not found")?;
        template.get_message(&context)
    }

    pub fn create_chat_from_template(&self, id: TemplateId, context: ChatTemplateContext) -> Result<()> {
        let message = self.get_message(id, context)?;
        self.chat_service.create_chat();
        self.chat_service.send_message(message)?;
        Ok(())
    }
}

impl ServiceApi for ChatTemplates {

}

impl ServiceInitializer for ChatTemplates {
    fn initialize(context: &AppContext) -> Arc<Self> {
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

