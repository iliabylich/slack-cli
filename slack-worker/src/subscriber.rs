use std::sync::mpsc;
use std::fmt;

use crate::{Message, SendError};

pub struct Subscriber {
    pub id: String,
    pub sender: mpsc::Sender<Message>
}

impl fmt::Debug for Subscriber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f
            .debug_struct("Subscriber")
            .field("id", &self.id)
            .finish()
    }
}

impl Subscriber {
    pub fn new(id: String, sender: mpsc::Sender<Message>) -> Self {
        Self { id, sender }
    }

    pub fn notify(&self, message: Message) -> Result<(), SendError> {
        self.sender.send(message).map_err(|err| SendError::new(&self.id, &err.to_string()) )
    }
}
