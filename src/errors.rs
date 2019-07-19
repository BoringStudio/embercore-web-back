use actix_web::{error::ResponseError, HttpResponse};

use std::fmt;

#[derive(Debug)]
pub enum ServiceError {
    InternalServerError,
    BadRequest(String),
    Unauthorized,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::InternalServerError => write!(f, "Internal server error"),
            ServiceError::BadRequest(ref message) => write!(f, "BadRequest: {}", message),
            ServiceError::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal server error. Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}
