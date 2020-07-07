extern crate reqwest;
extern crate serde_json;

mod error;
pub use error::Error as SlackError;

pub type SlackResult<T> = std::result::Result<T, SlackError>;

#[macro_use]
mod response;

mod http_client;
pub(crate) use http_client::HttpClient;

mod json_client;
pub(crate) use json_client::JsonClient;

mod slack_client;
pub use slack_client::SlackClient;

mod conversation;
pub use conversation::Conversation;

mod message;
pub use message::Message;

mod user;
pub use user::User;

#[macro_use]
extern crate http_resource;

mod slack_state;
pub use slack_state::SlackState;

mod worker;
pub use worker::SlackWorker;
