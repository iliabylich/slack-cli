use serde::de::DeserializeOwned;
use serde_json;

use crate::{Error, HttpClient, DefaultHttpClient};

pub struct JsonClient {
    pub api_prefix: String,
    pub http_client: Box<dyn HttpClient>
}

impl JsonClient {
    pub fn new(token: String, api_prefix: String) -> Result<Self, Error> {
        // let token = String::from("");
        println!("Using token {}", token);
        let http_client = DefaultHttpClient::new(token)?;
        Ok(Self { http_client: Box::new(http_client), api_prefix })
    }

    #[cfg(test)]
    pub fn new_with_http_client(http_client: Box<dyn HttpClient>) -> Self {
        Self { api_prefix: String::from(""), http_client }
    }

    pub fn get_json<T>(&self, method: &str) -> Result<T, Error> where T: DeserializeOwned {
        let url = format!("{}/{}", self.api_prefix, method);
        println!("Using url {}", url);

        let body = self.http_client.get(url)?;
        let json = serde_json::from_str::<T>(&body)?;
        Ok(json)
    }
}
