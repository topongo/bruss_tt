use std::error::Error;
use std::fmt::{Debug, Display};

use crate::api_error::ApiError;

#[derive(Debug)]
pub enum TTError {
    HttpError(String),
    JsonError(String),
    ApiError(ApiError)
}

impl From<reqwest::Error> for TTError {
    fn from(value: reqwest::Error) -> Self {
        if value.is_decode() {
            TTError::JsonError(value.to_string())
        } else {
            TTError::HttpError(value.to_string())
        }
    }
}

impl From<serde_json::Error> for TTError {
     fn from(value: serde_json::Error) -> Self {
         TTError::JsonError(value.to_string())
     }
}

impl Error for TTError {}

impl Display for TTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type TTResult<T> = Result<T, TTError>;

