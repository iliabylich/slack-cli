use std::env;

use crate::{JsonClient, Response, Error, Conversation, User, Message};

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

    pub fn list_conversations(&self) -> Result<Vec<Conversation>, Error> {
        use crate::conversation_meta::list::{Response, METHOD};
        self.json_client.get_json::<Response>(METHOD)?.to_result()
    }

    pub fn find_conversation(&self, conversation_id: &str) -> Result<Conversation, Error> {
        use crate::conversation_meta::find::{Response, METHOD};
        let method = format!("{}?channel={}", METHOD, conversation_id);
        self.json_client.get_json::<Response>(&method)?.to_result()
    }

    pub fn conversation_history(&self, conversation_id: &str) -> Result<Vec<Message>, Error> {
        use crate::conversation_meta::history::{Response, METHOD};
        let method = format!("{}?channel={}", METHOD, conversation_id);
        self.json_client.get_json::<Response>(&method)?.to_result()
    }

    pub fn list_users(&self) -> Result<Vec<User>, Error> {
        use crate::user_meta::list::{Response as ListUsersResponse, METHOD as LIST_USERS};
        self.json_client.get_json::<ListUsersResponse>(LIST_USERS)?.to_result()
    }
}
