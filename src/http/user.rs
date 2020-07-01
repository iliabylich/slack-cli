use serde::{Deserialize, Serialize};
use crate::http::{Response as HttpResponse, Error};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub real_name: String
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
            fn to_result(&self) -> Result<Vec<User>, Error> {
                if self.ok {
                    if let Some(users) = &self.members {
                        return Ok(users.clone());
                    }
                }
                if let Some(err) = &self.error {
                    return Err(Error::from(err))
                }
                Err(Error::from("Broken response format"))
            }
        }
    }
}
