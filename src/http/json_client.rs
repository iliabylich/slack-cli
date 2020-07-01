use reqwest::{blocking, header};
use serde::de::DeserializeOwned;
use serde_json;

pub struct JsonClient {
    api_prefix: String,
    http_client: blocking::Client
}

impl JsonClient {
    pub fn new(token: String, api_prefix: String) -> Result<Self, String> {
        // let token = String::from("");
        println!("Using token {}", token);
        let mut headers = header::HeaderMap::new();
        let header = header::HeaderValue::from_str(&format!("Bearer {}", token));
        match header {
            Ok(header) => {
                headers.insert(header::AUTHORIZATION, header);
                let http_client = blocking::Client::builder().default_headers(headers).build();
                match http_client {
                    Ok(http_client) => Ok(Self { http_client, api_prefix }),
                    Err(err) => Err(err.to_string())
                }
            },
            Err(err) => Err(err.to_string())
        }
    }

    pub fn get_json<T>(&self, method: &str) -> Result<T, String> where T: DeserializeOwned {
        let url = format!("{}/{}", self.api_prefix, method);
        println!("Using url {}", url);

        let err: String = match self.http_client.get(&url).send() {
            Ok(response) => {
                match response.text() {
                    Ok(body) => {
                        match serde_json::from_str::<T>(&body) {
                            Ok(json) => return Ok(json),
                            Err(err) => err.to_string()
                        }
                    },
                    Err(err) => err.to_string()
                }
            },
            Err(err) => err.to_string()
        };

        Err(format!("{} - {}", url, err))
    }
}
