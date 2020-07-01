use serde::{Deserialize, Serialize};
use crate::http::{Response as HttpResponse, Error};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: String
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
