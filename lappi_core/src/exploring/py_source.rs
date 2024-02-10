use serde::{Deserialize, Serialize};
use amina_core::service::Service;
use crate::exploring::{ExploringError, ExploringResult, ExploringSource};
use crate::py_server_client::{PyApiError, PyApiResult, PyServerClient};

impl From<PyApiError> for ExploringError {
    fn from(error: PyApiError) -> Self {
        match error {
            PyApiError::ConnectionError => ExploringError::ConnectionError,
            PyApiError::RequestError(message) => ExploringError::RequestError(message),
            PyApiError::DeserializationError(message) => ExploringError::RequestError("Deserialization error: ".to_string() + message.as_str()),
        }
    }
}

pub struct PyExploringSource {
    pub source_name: String,
    pub py_server_client: Service<PyServerClient>,
}

impl PyExploringSource {
    pub fn new(source_name: String, py_server_client: Service<PyServerClient>) -> Self {
        PyExploringSource {
            source_name,
            py_server_client,
        }
    }
}

impl ExploringSource for PyExploringSource {

    fn source_name(&self) -> String {
        self.source_name.clone()
    }

    fn get_artist_description(&self, artist_name: &str) -> ExploringResult<String> {
        #[derive(Serialize)]
        struct Req {
            source_name: String,
            artist_name: String,
        }
        #[derive(Deserialize)]
        struct Resp {
            description: String,
        }
        let resp: PyApiResult<Resp> = self.py_server_client.make_request("exploring.sources.get_artist_description", &Req {
            source_name: self.source_name.clone(),
            artist_name: artist_name.to_string()
        });
        return resp.map(|r| r.description).map_err(|e| e.into());
    }

    fn clone_source(&self) -> Box<dyn ExploringSource> {
        Box::new(PyExploringSource {
            source_name: self.source_name.clone(),
            py_server_client: self.py_server_client.clone(),
        })
    }

}
