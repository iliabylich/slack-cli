extern crate reqwest;
extern crate serde_json;

mod response;
pub use response::Response;

mod error;
pub use error::Error;

mod http_client;
pub use http_client::{HttpClient, DefaultHttpClient};

mod json_client;
pub use json_client::JsonClient;

mod slack_client;
pub use slack_client::SlackClient;

mod channel;
pub use channel::{Channel, meta as channel_meta};

mod user;
pub use user::{User, meta as user_meta};

#[cfg(test)]
pub use http_client::test_helper as http_helper;


#[cfg(test)]
mod tests {
    #[test]
    fn it_lists_channels() {
        assert_eq!(1, 1);
    }
}
