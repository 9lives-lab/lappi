use anyhow::Result;
use reqwest::blocking::Client;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompletionMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CompletionReq {
    pub model: String,
    pub messages: Vec<CompletionMessage>
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CompletionChoice {
    pub finish_reason: String,
    pub index: u32,
    pub message: CompletionMessage,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CompletionUsage {
    pub completion_tokens: u32,
    pub prompt_tokens: u32,
    pub total_tokens: u32
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CompletionResp {
    pub choices: Vec<CompletionChoice>,
    pub created: u32,
    pub id: String,
    pub model: String,
    pub object: String,
    pub usage: CompletionUsage
}

pub struct ChatGptApi {
    api_key: String,
}

impl ChatGptApi {
    pub fn new(api_key: String) -> ChatGptApi {
        ChatGptApi {
            api_key
        }
    }

    pub fn get_completion(&self, request: &CompletionReq) -> Result<CompletionResp> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", &self.api_key).parse()?);

        let client = Client::new();
        let response: CompletionResp = client.post("https://api.openai.com/v1/chat/completions")
            .headers(headers)
            .json(request)
            .send()?
            .json()?;

        Ok(response)
    }
}
