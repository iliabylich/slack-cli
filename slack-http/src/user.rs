use serde::{Deserialize, Serialize};
use crate::{Response as HttpResponse, SlackError};

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub real_name: String
}

impl User {
    #[cfg(test)]
    pub fn new(id: &str, name: &str, real_name: &str) -> Self {
        Self { id: id.to_owned(), name: name.to_owned(), real_name: real_name.to_owned() }
    }
}

pub mod meta {
    use super::*;
    use serde::{Deserialize, Serialize};

    pub mod list {
        use super::*;
        pub const METHOD: &str = "users.list";

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub ok: bool,
            pub error: Option<String>,
            pub members: Option<Vec<User>>,
        }

        impl HttpResponse<Vec<User>> for Response {
            fn to_result(&self) -> Result<Vec<User>, SlackError> {
                if self.ok {
                    if let Some(users) = &self.members {
                        return Ok(users.clone());
                    } else {
                        return Err(SlackError::from("'ok' is true, but 'members' is null"))
                    }
                }
                if let Some(err) = &self.error {
                    return Err(SlackError::from(err))
                }
                Err(SlackError::from("Broken response format (no 'error' field)"))
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_helper::get_test_slack_client;

    #[test]
    fn it_lists_users() {
        let request = "/users.list";
        let response = r#"
            {
                "ok": true,
                "members": [
                    {
                        "id": "1",
                        "name": "me",
                        "real_name": "Me"
                    },
                    {
                        "id": "2",
                        "name": "you",
                        "real_name": "You"
                    }
                ]
            }
        "#;


        let slack = get_test_slack_client(request, response);
        let result = slack.list_users().unwrap();

        assert_eq!(
            result,
            vec![
                User::new("1", "me", "Me"),
                User::new("2", "you", "You"),
            ]
        )
    }
}
