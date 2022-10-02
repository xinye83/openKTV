use crate::ApiError;

pub mod consts;


pub trait OptionUtil {
    fn guard(&self) -> Result<String, ApiError>;
}

impl OptionUtil for Option<String> {
    fn guard(self: &Self) -> Result<String, ApiError> {
        match self {
            None => Err(ApiError::BadClientData),
            Some(it) => Ok(it.clone())
        }
    }
}