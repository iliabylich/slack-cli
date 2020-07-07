#[derive(Debug)]
pub struct SendError {
    subscriber_id: String,
    message: String
}

impl SendError {
    pub fn new(subscriber_id: &str, message: &str) -> Self {
        Self { subscriber_id: String::from(subscriber_id), message: String::from(message) }
    }
}
