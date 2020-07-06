use crate::{SlackResult};
pub(crate) trait Response<T>{
    fn to_result(&self) -> SlackResult<T>;
}
