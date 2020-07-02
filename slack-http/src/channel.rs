use serde::{Deserialize, Serialize};
use crate::{Response as HttpResponse, Error};

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: String
}

impl Channel {
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
            pub channels: Option<Vec<Channel>>,
        }

        impl HttpResponse<Vec<Channel>> for Response {
            fn to_result(&self) -> Result<Vec<Channel>, Error> {
                if self.ok {
                    if let Some(channels) = &self.channels {
                        return Ok(channels.clone());
                    } else {
                        return Err(Error::from("'ok' is true, but 'channels' is null"))
                    }
                }
                if let Some(err) = &self.error {
                    return Err(Error::from(err))
                }
                Err(Error::from("Broken response format (no 'error' field)"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_helper::get_test_slack_client;

    #[test]
    fn it_lists_channels() {
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
        let result = slack.list_channels().unwrap();

        assert_eq!(
            result,
            vec![
                Channel::new("42", "GitHub"),
                Channel::new("17", "Work"),
            ]
        )
    }
}
