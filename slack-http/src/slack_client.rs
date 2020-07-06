use std::env;

use crate::{JsonClient, Response, SlackError, SlackResult, Conversation, User, Message};

pub struct SlackClient {
    pub json_client: JsonClient
}

const API_PREFIX: &str = "https://slack.com/api";

impl SlackClient {
    pub fn new(token: &str) -> SlackResult<Self> {
        let json_client = JsonClient::new(token, API_PREFIX)?;
        Ok(Self { json_client })
    }

    pub fn new_from_env() -> SlackResult<Self> {
        let token = env::var("SLACK_TOKEN").map_err(|_err|
            SlackError::from("No SLACK_TOKEN env variable")
        )?;

        Self::new(&token)
    }

    #[cfg(test)]
    pub fn new_with_json_client(json_client: JsonClient) -> Self {
        Self { json_client }
    }

    pub fn list_conversations(&self) -> SlackResult<Vec<Conversation>> {
        use crate::conversation_meta::list::{Response, METHOD};
        self.json_client.get_json::<Response>(METHOD)?.to_result()
    }

    pub fn find_conversation(&self, conversation_id: &str) -> SlackResult<Conversation> {
        use crate::conversation_meta::find::{Response, METHOD};
        let method = format!("{}?channel={}", METHOD, conversation_id);
        self.json_client.get_json::<Response>(&method)?.to_result()
    }

    pub fn conversation_history(&self, conversation_id: &str) -> SlackResult<Vec<Message>> {
        use crate::conversation_meta::history::{Response, METHOD};
        let method = format!("{}?channel={}", METHOD, conversation_id);
        self.json_client.get_json::<Response>(&method)?.to_result()
    }

    pub fn list_users(&self) -> SlackResult<Vec<User>> {
        use crate::user_meta::list::{Response as ListUsersResponse, METHOD as LIST_USERS};
        self.json_client.get_json::<ListUsersResponse>(LIST_USERS)?.to_result()
    }
}
