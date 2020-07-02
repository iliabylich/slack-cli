use reqwest::{header};
use serde_json;

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub url: Option<String>
}

impl Error {
    pub fn new(message: String) -> Self {
        Self { message, url: Option::None }
    }

    pub fn with_url(&self, url: String) -> Self {
        Self { message: self.message.clone(), url: Option::Some(url) }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        let message = e.to_string().to_owned();
        let url = e.url().map(|url| url.to_string() );
        Error { message, url }
    }
}

impl From<header::InvalidHeaderValue> for Error {
    fn from(e: header::InvalidHeaderValue) -> Self {
        Error::new(e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::new(e.to_string())
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::new(e)
    }
}

impl From<&String> for Error {
    fn from(e: &String) -> Self {
        Error::new(e.clone())
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Error::new(e.to_owned())
    }
}
