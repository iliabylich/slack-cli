use crate::http::Error;
pub trait Response<T>{
    fn to_result(&self) -> Result<T, Error>;
}
