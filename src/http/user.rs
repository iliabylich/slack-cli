use serde::{Deserialize, Serialize};
use crate::http::{Response as HttpResponse};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub real_name: String
}

pub mod meta {
    use super::{User, HttpResponse};
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
            fn to_result(&self) -> Result<Vec<User>, String> {
                println!("{:#?}", self);
                if self.ok {
                    if let Some(users) = &self.members {
                        return Ok(users.clone());
                    }
                } else {
                    if let Some(err) = &self.error {
                        return Err(err.clone())
                    }
                }
                Err(String::from("Broken response format"))
            }
        }
    }
}
