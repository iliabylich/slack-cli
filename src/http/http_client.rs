use reqwest::{blocking, header};

use crate::http::Error;

pub trait HttpClient {
    fn get(&self, url: String) -> Result<String, Error>;
}

pub struct DefaultHttpClient {
    http_client: blocking::Client
}

impl DefaultHttpClient {
    pub fn new(token: String) -> Result<Self, Error> {
        let mut headers = header::HeaderMap::new();
        let header = header::HeaderValue::from_str(&format!("Bearer {}", token))?;
        headers.insert(header::AUTHORIZATION, header);
        let http_client = blocking::Client::builder().default_headers(headers).build()?;
        Ok(Self { http_client })
    }
}

impl HttpClient for DefaultHttpClient {
    fn get(&self, url: String) -> Result<String, Error> {
        let response = self.http_client.get(&url).send().map_err(|err| Error::from(err).with_url(url) )?;
        let body = response.text()?;
        Ok(body)
    }
}

#[cfg(test)]
pub mod test_helper {
    use super::*;

    use std::collections::HashMap;
    use crate::http::{SlackClient, JsonClient};

    #[derive(Hash, Eq, PartialEq, Debug)]
    pub struct FakeRequest {
        url: String
    }

    impl FakeRequest {
        pub fn new(url: &str) -> Self {
            Self { url: String::from(url) }
        }
    }

    #[derive(Hash, Eq, PartialEq, Debug)]
    pub struct FakeResponse {
        body: String
    }

    impl FakeResponse {
        pub fn new(body: &str) -> Self {
            Self { body: String::from(body) }
        }
    }

    pub struct TestHttpClient {
        map: HashMap<FakeRequest, FakeResponse>
    }

    impl TestHttpClient {
        pub fn new() -> Self {
            Self { map: HashMap::new() }
        }

        pub fn add(&mut self, request: FakeRequest, response: FakeResponse) {
            self.map.insert(request, response);
        }

        pub fn find(&self, url: String) -> Option<&FakeResponse> {
            self.map.get(&FakeRequest { url })
        }
    }

    impl HttpClient for TestHttpClient {
        fn get(&self, url: String) -> Result<String, Error> {
            if let Some(response) = self.find(url.clone()) {
                Ok(response.body.clone())
            } else {
                Err(Error::from(format!("404 for {}", url)))
            }
        }
    }

    pub fn get_test_slack_client(request: &str, response: &str) -> SlackClient {
        let mut test_http_client = TestHttpClient::new();
        test_http_client.add(
            FakeRequest::new(request),
            FakeResponse::new(response)
        );
        let json_client = JsonClient::new_with_http_client(Box::new(test_http_client));
        let slack_client = SlackClient::new_with_json_client(json_client);
        slack_client
    }
}
