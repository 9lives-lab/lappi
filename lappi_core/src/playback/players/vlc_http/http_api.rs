use std::path::Path;

use reqwest::blocking::Client;
use reqwest::Result as ReqwestResult;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusResponse {
    pub position: f64,
    pub length: i32,
    pub state: String,
}

pub struct VlcHttpApi {

}

impl VlcHttpApi {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn send_command(&self, args: &[(&str, &str)]) -> ReqwestResult<StatusResponse> {
        let client = Client::new();
        let mut url = "http://localhost:9090/requests/status.json?".to_string();
        for (key, value) in args {
            url.push_str(format!("{}={}&", key, value).as_str());
        }
        url.pop();
        let response: StatusResponse = client.get(url.as_str())
            .basic_auth("", Some("123"))
            .send()?
            .json()?;
        return Ok(response);
    }

    pub fn get_status(&self) -> ReqwestResult<StatusResponse> {
        self.send_command(&[])
    }

    pub fn play_file(&self, file_path: &Path) -> ReqwestResult<StatusResponse> {
        let url = Url::from_file_path(file_path).expect("Invalid file path");
        self.send_command(&[
            ("command", "in_play"),
            ("input", url.as_str()),
        ])
    }

    pub fn resume(&self) -> ReqwestResult<StatusResponse> {
        self.send_command(&[
            ("command", "pl_forceresume"),
        ])
    }

    pub fn pause(&self) -> ReqwestResult<StatusResponse> {
        self.send_command(&[
            ("command", "pl_forcepause"),
        ])
    }

    pub fn seek(&self, progress: i32) -> ReqwestResult<StatusResponse> {
        self.send_command(&[
            ("command", "seek"),
            ("val", &format!("{}", progress))
        ])
    }
}

