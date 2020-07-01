use std::env;

use crate::http::{JsonClient, Response, Channel, User};

pub struct SlackClient {
    json_client: JsonClient
}

const API_PREFIX: &str = "https://slack.com/api";

impl SlackClient {
    pub fn new(token: String) -> Result<Self, String> {
        let json_client = JsonClient::new(token, String::from(API_PREFIX))?;
        Ok(Self { json_client })
    }

    pub fn new_from_env() -> Result<Self, String> {
        match env::var("SLACK_TOKEN") {
            Ok(value) => Self::new(value),
            Err(err) => Err(err.to_string())
        }
    }

    pub fn list_channels(&self) -> Result<Vec<Channel>, String> {
        use crate::http::channel_meta::list::{Response as ListChannelsResponse, METHOD as LIST_CHANNELS};
        self.json_client.get_json::<ListChannelsResponse>(LIST_CHANNELS)?.to_result()
    }

    pub fn list_users(&self) -> Result<Vec<User>, String> {
        use crate::http::user_meta::list::{Response as ListUsersResponse, METHOD as LIST_USERS};
        self.json_client.get_json::<ListUsersResponse>(LIST_USERS)?.to_result()
    }
}
