mod http_api;

use std::sync::{Arc, Mutex};

use amina_core::service::Context;
use amina_core::settings::Property;
use anyhow::Result;

use crate::exploring::chat::{ChatApi, ChatFactory, ChatRole, Message};
use crate::exploring::chat::chatgpt::http_api::CompletionMessage;
use crate::settings::Settings;

pub struct ChatGpt {
    api_key: Property<String>,
    dialog: Mutex<Vec<CompletionMessage>>
}

impl ChatApi for ChatGpt {
    fn send_message(&self, message: String) -> Result<()> {
        let api = http_api::ChatGptApi::new(self.api_key.get());

        let mut dialog = self.dialog.lock().unwrap();
        dialog.push(CompletionMessage {
            role: "user".to_string(),
            content: message
        });

        let request = http_api::CompletionReq {
            model: "gpt-3.5-turbo".to_string(),
            messages: dialog.clone()
        };
        let response = api.get_completion(&request)?;

        dialog.push(response.choices[0].message.clone());

        Ok(())
    }

    fn get_dialog(&self) -> Vec<Message> {
        let dialog = self.dialog.lock().unwrap();
        let mut messages = vec![];
        for message in dialog.iter() {
            match message.role.as_str() {
                "user" => messages.push(Message {
                    role: ChatRole::User,
                    content: message.content.clone()
                }),
                "assistant" => messages.push(Message {
                    role: ChatRole::Assistant,
                    content: message.content.clone()
                }),
                _ => {}
            }
        }
        return messages;
    }
}

pub struct ChatGptFactory {
    api_key: Property<String>
}

impl ChatGptFactory {
    pub fn new(context: &Context) -> ChatGptFactory {
        let settings = context.get_service::<Settings>();
        let api_key = settings.get_string("exploring.sources.chatgpt.user_token");
        ChatGptFactory {
            api_key,

        }
    }

}

impl ChatFactory for ChatGptFactory {
    fn create_chat(&self) -> Arc<dyn ChatApi> {
        let dialog = vec![CompletionMessage {
            role: "system".to_string(),
            content: "You are a helpful assistant.".to_string()
        }];
        Arc::new(ChatGpt {
            api_key: self.api_key.clone(),
            dialog: Mutex::new(dialog),
        })
    }
}