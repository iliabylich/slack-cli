use reqwest::{header};
use serde_json;

#[derive(Debug, PartialEq)]
pub struct Error {
    pub message: String,
    pub url: Option<String>
}

impl Error {
    pub fn new(message: &str) -> Self {
        Self { message: String::from(message), url: Option::None }
    }

    pub fn with_url(&self, url: &str) -> Self {
        Self { message: self.message.clone(), url: Option::Some(String::from(url)) }
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
        Error::new(&e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::new(&e.to_string())
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::new(&e)
    }
}

impl From<&String> for Error {
    fn from(e: &String) -> Self {
        Error::new(&e)
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Error::new(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_new() {
        let error = Error::new("message");
        assert_eq!(error, Error { message: String::from("message"), url: None });
    }

    #[test]
    fn test_with_url() {
        let error = Error::new("message").with_url("https://example.com");
        assert_eq!(error.url, Some(String::from("https://example.com")));
    }

    #[test]
    fn error_from_string() {
        let error = Error::from(String::from("String"));
        assert_eq!(error, Error { message: String::from("String"), url: None })
    }

    #[test]
    fn error_from_str() {
        let error = Error::from("str");
        assert_eq!(error, Error { message: String::from("str"), url: None })
    }
}
