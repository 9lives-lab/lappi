use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use serde::{Deserialize, Serialize};

use amina_core::service::{Context, ServiceApi, ServiceInitializer};

#[derive(Deserialize)]
struct RequestError {
    message: String,
}

#[derive(Deserialize)]
struct RequestResult<R> {
    data: Option<R>,
    error: Option<RequestError>,
}

#[derive(Deserialize, Debug)]
pub enum PyApiError {
    ConnectionError,
    DeserializationError(String),
    RequestError(String),
}

pub type PyApiResult<T> = Result<T, PyApiError>;

#[derive(Deserialize)]
pub struct EmptyResponse {
    empty_response: bool
}

impl EmptyResponse {
    pub fn validate(&self) -> PyApiResult<()> {
        if self.empty_response {
            Ok(())
        } else {
            Err(PyApiError::DeserializationError("Empty response validation failed".to_string()))
        }
    }
}

pub struct PyServerClient {
    is_connected: bool,
}

impl PyServerClient {

    pub fn check_connection(&self) -> PyApiResult<()> {
        #[derive(Serialize, Deserialize)]
        struct ReqResp {
            test_field: bool,
        }
        let _: ReqResp = self.make_request("check_connection", &ReqResp {
            test_field: false,
        })?;

        return Ok(());
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected
    }

    pub fn set_registry_values(&self, values: HashMap<String, String>) -> PyApiResult<()> {
        #[derive(Serialize)]
        struct Req {
            registry_values: HashMap<String, String>,
        }
        let resp: PyApiResult<EmptyResponse> = self.make_request("registry.set_values", &Req {
            registry_values: values,
        });

        return resp?.validate();
    }

    pub fn set_registry_value(&self, key: &str, value: String) -> PyApiResult<()> {
        let mut values = HashMap::new();
        values.insert(key.to_string(), value);
        self.set_registry_values(values)
    }

    pub fn make_request<O, I>(&self, key: &str, req: &I) -> PyApiResult<O> where
        for<'de> O: Deserialize<'de> + Send + 'static,
        I: Serialize + Send + 'static,
    {
        let client = reqwest::blocking::Client::builder().timeout(Duration::from_secs(10*60)).build().unwrap();
        let res = client.post("http://localhost:5000/make_request?key=".to_string() + key)
            .json(req)
            .send();
        match res {
            Ok(resp) => {
                match resp.json::<RequestResult<O>>() {
                    Ok(result) => {
                        if let Some(result) = result.data {
                            Ok(result)
                        } else {
                            if let Some(error) = result.error {
                                Err(PyApiError::RequestError(error.message))
                            } else {
                                Err(PyApiError::DeserializationError("Empty response".to_string()))
                            }
                        }
                    },
                    Err(err) => {
                        Err(PyApiError::DeserializationError(err.to_string()))
                    }
                }
            },
            Err(_) => {
                Err(PyApiError::ConnectionError)
            }
        }
    }

}

impl ServiceApi for PyServerClient {

}

impl ServiceInitializer for PyServerClient {

    fn initialize(_: &Context) -> Arc<Self> {
        let mut py_server_client = Self {
            is_connected: false,
        };

        if let Err(err) = py_server_client.check_connection() {
            log::error!("Failed to connect to python server: {:?}", err);
        } else {
            py_server_client.is_connected = true;
        }

        return Arc::new(py_server_client);
    }

}
