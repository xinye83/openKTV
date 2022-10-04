use crate::api::ApiError;

pub mod consts;
pub mod tube_utils;
pub mod iina_utils;

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

fn take<T>(vec: Vec<T>, index: usize) -> Option<T> {
    vec.into_iter().nth(index)
}