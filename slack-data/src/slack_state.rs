use crate::{User, Conversation};

#[derive(Debug)]
pub struct SlackState {
    pub uptime: i32,

    pub all_users: Vec<User>,
    pub all_conversations: Vec<Conversation>,

    pub current_conversation_id: Option<String>,
}

impl Default for SlackState {
    fn default() -> Self {
        Self {
            uptime: 0,

            all_users: vec![],
            all_conversations: vec![],

            current_conversation_id: None
        }
    }
}
