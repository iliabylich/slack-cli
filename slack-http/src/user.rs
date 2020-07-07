use serde::{Deserialize, Serialize};
use crate::response::{Response as HttpResponse};

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

pub(crate) mod meta {
    use super::*;
    use serde::{Deserialize, Serialize};

    pub mod list {
        use super::*;
        pub const METHOD: &str = "users.list";

        #[derive(Debug, Serialize, Deserialize, HttpResource)]
        pub struct Response {
            pub ok: bool,
            pub error: Option<String>,
            #[result]
            pub members: Option<Vec<User>>,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_client::test_helper::get_test_slack_client;

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
        let result = match slack.list_users() {
            Ok(value) => value,
            Err(err) => panic!("got err: {:#?}", err)
        };

        assert_eq!(
            result,
            vec![
                User::new("1", "me", "Me"),
                User::new("2", "you", "You"),
            ]
        )
    }
}
