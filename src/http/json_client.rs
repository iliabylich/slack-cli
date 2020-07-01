use reqwest::{blocking, header};
use serde::de::DeserializeOwned;
use serde_json;

use crate::http::Error;

pub struct JsonClient {
    api_prefix: String,
    http_client: blocking::Client
}

impl JsonClient {
    pub fn new(token: String, api_prefix: String) -> Result<Self, Error> {
        // let token = String::from("");
        println!("Using token {}", token);
        let mut headers = header::HeaderMap::new();
        let header = header::HeaderValue::from_str(&format!("Bearer {}", token))?;
        headers.insert(header::AUTHORIZATION, header);
        let http_client = blocking::Client::builder().default_headers(headers).build()?;
        Ok(Self { http_client, api_prefix })
    }

    pub fn get_json<T>(&self, method: &str) -> Result<T, Error> where T: DeserializeOwned {
        let url = format!("{}/{}", self.api_prefix, method);
        println!("Using url {}", url);

        let response = self.http_client.get(&url).send().map_err(|err| Error::from(err).with_url(url) )?;
        let body = response.text()?;
        let json = serde_json::from_str::<T>(&body)?;
        Ok(json)
    }
}
