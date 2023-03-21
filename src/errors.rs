use actix_web::{error::ResponseError, HttpResponse};
use diesel::result::Error as DieselError;
use std::fmt;

#[derive(Debug)]
pub struct CustomError {
    inner: DieselError,
}

impl From<DieselError> for CustomError {
    fn from(error: DieselError) -> Self {
        CustomError { inner: error }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json(format!("Internal Server Error: {}", self.inner))
    }
}