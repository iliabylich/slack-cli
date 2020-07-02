extern crate reqwest;
extern crate serde_json;

mod error;
pub use error::Error as SlackError;

pub type SlackResult<T> = std::result::Result<T, SlackError>;

#[macro_use]
mod response;
pub use response::Response;

mod http_client;
pub use http_client::{HttpClient, DefaultHttpClient};

mod json_client;
pub use json_client::JsonClient;

mod slack_client;
pub use slack_client::SlackClient;

mod conversation;
pub use conversation::{Conversation, meta as conversation_meta};

mod message;
pub use message::{Message, meta as message_meta};

mod user;
pub use user::{User, meta as user_meta};

#[cfg(test)]
pub use http_client::test_helper as http_helper;

#[macro_use]
extern crate http_resource;
