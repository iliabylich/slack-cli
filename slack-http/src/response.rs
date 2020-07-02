use crate::{SlackResult};
pub trait Response<T>{
    fn to_result(&self) -> SlackResult<T>;
}

#[macro_export]
macro_rules! define_conversion_to_result {
    ($response: ty, $field: ident: $type: ty) => {
        impl HttpResponse<$type> for $response {
            fn to_result(&self) -> $crate::SlackResult<$type> {
                if self.ok {
                    if let Some(channels) = &self.$field {
                        return Ok(channels.clone());
                    } else {
                        return Err($crate::SlackError::from(format!("'ok' is true, but '{}' is null", stringify!($field))));
                    }
                }
                if let Some(err) = &self.error {
                    return Err($crate::SlackError::from(err));
                }
                Err($crate::SlackError::from("Broken response format (no 'error' field)"))
            }
        }
    };
}
