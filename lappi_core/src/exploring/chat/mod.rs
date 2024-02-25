mod chatgpt;

use std::sync::{Arc, Mutex};

use amina_core::register_rpc_handler;
use amina_core::rpc::Rpc;
use amina_core::service::{Context, ServiceApi, ServiceInitializer};

use crate::exploring::chat::chatgpt::ChatGptFactory;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ChatRole {
    Assistant,
    User,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    role: ChatRole,
    content: String,
}
pub trait ChatApi: Send + Sync {
    fn send_message(&self, message: String);
    fn get_dialog(&self) -> Vec<Message>;
}

pub trait ChatFactory: Send + Sync {
    fn create_chat(&self) -> Arc<dyn ChatApi>;
}

pub struct ChatService {
    chat_factory: Mutex<Arc<dyn ChatFactory>>,
    current_chat: Mutex<Arc<dyn ChatApi>>,
}

impl ChatService {
    pub fn create_chat(&self) {
        let factory = self.chat_factory.lock().unwrap();
        let chat = factory.create_chat();
        let mut current_chat = self.current_chat.lock().unwrap();
        *current_chat = chat;
    }

    pub fn send_message(&self, message: String) -> Vec<Message> {
        let chat = self.current_chat.lock().unwrap();
        chat.send_message(message);
        return chat.get_dialog();
    }

    pub fn get_dialog(&self) -> Vec<Message> {
        let chat = self.current_chat.lock().unwrap();
        chat.get_dialog()
    }
}

impl ServiceApi for ChatService {

}

impl ServiceInitializer for ChatService {
    fn initialize(context: &Context) -> Arc<Self> {
        let rpc = context.get_service::<Rpc>();
        let factory = ChatGptFactory::new(context);
        let current_chat = factory.create_chat();
        let chat = Arc::new(ChatService {
            chat_factory: Mutex::new(Arc::new(factory)),
            current_chat: Mutex::new(current_chat.clone()),
        });

        register_rpc_handler!(rpc, chat, "lappi.chat.create_chat", create_chat());
        register_rpc_handler!(rpc, chat, "lappi.chat.send_message", send_message(message: String));
        register_rpc_handler!(rpc, chat, "lappi.chat.get_dialog", get_dialog());

        return chat;
    }
}

