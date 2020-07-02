use serde::{Deserialize, Serialize};
use crate::{Response as HttpResponse, SlackError};

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub name: String
}

impl Conversation {
    #[cfg(test)]
    pub fn new(id: &str, name: &str) -> Self {
        Self { id: id.to_owned(), name: name.to_owned() }
    }
}

pub mod meta {
    use super::*;
    use serde::{Deserialize, Serialize};

    pub mod list {
        use super::*;
        pub const METHOD: &str = "conversations.list";


        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub ok: bool,
            pub error: Option<String>,
            pub channels: Option<Vec<Conversation>>,
        }

        define_conversion_to_result!(Response, channels: Vec<Conversation>);
    }

    pub mod find {
        use super::*;
        pub const METHOD: &str = "conversations.info";

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub ok: bool,
            pub error: Option<String>,
            pub channel: Option<Conversation>,
        }

        define_conversion_to_result!(Response, channel: Conversation);
    }

    pub mod history {
        use super::*;
        pub const METHOD: &str = "conversations.history";
        use crate::Message;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub ok: bool,
            pub error: Option<String>,
            pub messages: Option<Vec<Message>>,
        }

        define_conversion_to_result!(Response, messages: Vec<Message>);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_helper::get_test_slack_client;

    #[test]
    fn it_lists_conversations() {
        let request = "/conversations.list";
        let response = r#"
            {
                "ok": true,
                "channels": [
                    {
                        "id": "42",
                        "name": "GitHub"
                    },
                    {
                        "id": "17",
                        "name": "Work"
                    }
                ]
            }
        "#;


        let slack = get_test_slack_client(request, response);
        let result = slack.list_conversations().unwrap();

        assert_eq!(
            result,
            vec![
                Conversation::new("42", "GitHub"),
                Conversation::new("17", "Work"),
            ]
        )
    }

    #[test]
    fn it_finds_conversation() {
        let request = "/conversations.info?channel=CHANNEL_ID";
        let response = r#"
            {
                "ok": true,
                "channel": {
                    "id": "CHANNEL_ID",
                    "name": "conversation name"
                }
            }
        "#;

        let slack = get_test_slack_client(request, response);
        let result = slack.find_conversation("CHANNEL_ID").unwrap();

        assert_eq!(
            result,
            Conversation::new("CHANNEL_ID", "conversation name")
        )
    }

    #[test]
    fn it_selects_conversations_history() {
        let request = "/conversations.history?channel=CHANNEL_ID";
        let response = r#"
            {
                "ok": true,
                "messages": [
                    {
                        "text": "message text1",
                        "user": "User1",
                        "ts": "42.001"
                    },
                    {
                        "text": "message text2",
                        "user": "User2",
                        "ts": "42.002"
                    }
                ]
            }
        "#;

        let slack = get_test_slack_client(request, response);
        let result = slack.conversation_history("CHANNEL_ID").unwrap();

        use crate::Message;

        assert_eq!(
            result,
            vec![
                Message::new("message text1", "User1", "42.001"),
                Message::new("message text2", "User2", "42.002"),
            ]
        )
    }
}
