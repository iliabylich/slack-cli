use serde::{Deserialize, Serialize};
use crate::http::{Response as HttpResponse};

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
            fn to_result(&self) -> Result<Vec<Channel>, String> {
                if self.ok {
                    if let Some(channels) = &self.channels {
                        return Ok(channels.clone());
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
