use crate::{SlackResult};
pub trait Response<T>{
    fn to_result(&self) -> SlackResult<T>;
}
