use std::env;

use crate::{JsonClient, Response, Channel, User, Error};

pub struct SlackClient {
    pub json_client: JsonClient
}

const API_PREFIX: &str = "https://slack.com/api";

impl SlackClient {
    pub fn new(token: String) -> Result<Self, Error> {
        let json_client = JsonClient::new(token, String::from(API_PREFIX))?;
        Ok(Self { json_client })
    }

    pub fn new_from_env() -> Result<Self, Error> {
        let token = env::var("SLACK_TOKEN").map_err(|_err|
            Error::from("No SLACK_TOKEN env variable")
        )?;

        Self::new(token)
    }

    #[cfg(test)]
    pub fn new_with_json_client(json_client: JsonClient) -> Self {
        Self { json_client }
    }

    pub fn list_channels(&self) -> Result<Vec<Channel>, Error> {
        use crate::channel_meta::list::{Response as ListChannelsResponse, METHOD as LIST_CHANNELS};
        self.json_client.get_json::<ListChannelsResponse>(LIST_CHANNELS)?.to_result()
    }

    pub fn find_channel(&self, channel_id: &str) -> Result<Channel, Error> {
        use crate::channel_meta::find::{Response as FindChannelResponse, METHOD as FIND_CHANNEL};
        let find_channel = format!("{}?channel={}", FIND_CHANNEL, channel_id);
        self.json_client.get_json::<FindChannelResponse>(&find_channel)?.to_result()
    }

    pub fn list_users(&self) -> Result<Vec<User>, Error> {
        use crate::user_meta::list::{Response as ListUsersResponse, METHOD as LIST_USERS};
        self.json_client.get_json::<ListUsersResponse>(LIST_USERS)?.to_result()
    }
}
