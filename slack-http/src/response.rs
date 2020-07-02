use crate::SlackError;
pub trait Response<T>{
    fn to_result(&self) -> Result<T, SlackError>;
}

#[macro_export]
macro_rules! define_conversion_to_result {
    ($response: ty, $field: ident: $type: ty) => {
        impl HttpResponse<$type> for $response {
            fn to_result(&self) -> Result<$type, SlackError> {
                if self.ok {
                    if let Some(channels) = &self.$field {
                        return Ok(channels.clone());
                    } else {
                        return Err(SlackError::from(format!("'ok' is true, but '{}' is null", stringify!($field))));
                    }
                }
                if let Some(err) = &self.error {
                    return Err(SlackError::from(err));
                }
                Err(SlackError::from("Broken response format (no 'error' field)"))
            }
        }
    };
}
