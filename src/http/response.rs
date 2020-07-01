pub trait Response<T>{
    fn to_result(&self) -> Result<T, String>;
}
