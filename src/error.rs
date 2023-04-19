use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde_json::json;
use serde_json::{Map as JsonMap, Value as JsonValue};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    BmkgwError(bmkgw::Error),
    NotFound(String),
}

// the ResponseError trait lets us convert errors to http responses with appropriate data
// https://actix.rs/docs/errors/
impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::NotFound(ref message) => {
                HttpResponse::NotFound().json(json!({ "message": message }))
            }
            Error::BmkgwError(ref message) => {
                HttpResponse::InternalServerError().json(json!({ "message": message.to_string() }))
            }
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::BmkgwError(ref x) => write!(f, "{}", x),
            Error::NotFound(ref x) => write!(f, "{}", x),
        }
    }
}

impl std::error::Error for Error {}

macro_rules! error_wrap {
    ($f:ty, $e:expr) => {
        impl From<$f> for Error {
            fn from(f: $f) -> Error {
                $e(f)
            }
        }
    };
}

error_wrap!(bmkgw::Error, Error::BmkgwError);
