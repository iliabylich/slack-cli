use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub text: String,
    pub user: String,
    pub ts: String
}

impl Message {
    #[cfg(test)]
    pub fn new(text: &str, user: &str, ts: &str) -> Self {
        Self { text: text.to_owned(), user: user.to_owned(), ts: ts.to_owned() }
    }
}

pub mod meta {

}
