use actix_web::http;
use reqwest::{self, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{config::Config, errors::Error};
pub mod posts;
pub mod users;

#[allow(dead_code)]
#[derive(Deserialize)]
struct ErrorResponse {
    status: u16,
    error: String,
    description: String,
}

#[derive(Clone)]
pub struct Client {
    reqwest: reqwest::Client,
    host_url: String,
}

impl Client {
    /// Create a CMS Client from Config
    pub fn from_config(config: &Config) -> Result<Client, Error> {
        let client = reqwest::Client::builder()
            .build()
            .map_err(|e| Error::Internal(e.to_string()))?;

        return Ok(Client {
            reqwest: client,
            host_url: config.backend_uri.clone(),
        });
    }

    /// Send a GET Request
    pub fn get(&self) -> ClientRequestBuilder {
        return ClientRequestBuilder {
            request: ClientRequest {
                client: self.clone(),
                path: String::new(),
                method: http::Method::GET,
                body: None,
            },
        };
    }

    /// Send a POST request
    pub fn post(&self) -> ClientRequestBuilder {
        return ClientRequestBuilder {
            request: ClientRequest {
                client: self.clone(),
                path: String::new(),
                method: http::Method::POST,
                body: None,
            },
        };
    }

    /// Send a PATCH Request
    pub fn patch(&self) -> ClientRequestBuilder {
        return ClientRequestBuilder {
            request: ClientRequest {
                client: self.clone(),
                path: String::new(),
                method: http::Method::PATCH,
                body: None,
            },
        };
    }

    /// Send a PUT Request
    pub fn put(&self) -> ClientRequestBuilder {
        return ClientRequestBuilder {
            request: ClientRequest {
                client: self.clone(),
                path: String::new(),
                method: http::Method::PUT,
                body: None,
            },
        };
    }

    /// Send a DELETE Request
    pub fn delete(&self) -> ClientRequestBuilder {
        return ClientRequestBuilder {
            request: ClientRequest {
                client: self.clone(),
                path: String::new(),
                method: http::Method::DELETE,
                body: None,
            },
        };
    }
}

pub struct ClientRequest {
    client: Client,
    path: String,
    method: reqwest::Method,
    body: Option<String>,
}

pub struct ClientRequestBuilder {
    request: ClientRequest,
}

impl ClientRequestBuilder {
    /// Set the path of the request
    pub fn path(&mut self, path: String) -> &mut Self {
        self.request.path = path;
        return self;
    }

    /// Set the JSON body of the request
    pub fn body<B>(&mut self, body: B) -> Result<&mut Self, Error>
    where
        B: Serialize,
    {
        let serialized =
            serde_json::to_string(&body).map_err(|e| Error::Internal(e.to_string()))?;
        self.request.body = Some(serialized);
        return Ok(self);
    }

    /// Send the request and parse the result as JSON
    pub async fn json<T>(&self) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let mut req = self.request.client.reqwest.request(
            self.request.method.clone(),
            format!("{}/{}", self.request.client.host_url, self.request.path),
        );

        if let Some(body) = &self.request.body {
            req = req
                .body(body.clone())
                .header("Content-Type", "application/json")
        }

        let response = req
            .send()
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;

        let response_error = response.error_for_status_ref().err();

        if let Some(e) = response_error {
            let error = response
                .json::<ErrorResponse>()
                .await
                .map_err(|f| Error::Internal(format!("Unknown Error: {}", f.to_string())))?;

            return Err(match e.status() {
                Some(status) => match status {
                    StatusCode::INTERNAL_SERVER_ERROR => Error::Internal(error.description),
                    StatusCode::BAD_REQUEST => Error::BadRequest(error.description),
                    StatusCode::NOT_FOUND => Error::NotFound(error.description),
                    StatusCode::UNAUTHORIZED => Error::Unauthorized(error.description),
                    StatusCode::FORBIDDEN => Error::Forbidden(error.description),
                    _ => Error::Internal(e.to_string()),
                },
                None => Error::Internal(e.to_string()),
            });
        }

        return response
            .json::<T>()
            .await
            .map_err(|e| Error::Internal(e.to_string()));
    }
}
