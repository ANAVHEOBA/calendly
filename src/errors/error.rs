use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use mongodb::error::Error as MongoError;
use serde::Serialize;

#[derive(Debug, Display, Serialize)]
pub enum AppError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError(String),

    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized: {}", _0)]
    Unauthorized(String),

    #[display(fmt = "Not Found: {}", _0)]
    NotFound(String),

    #[display(fmt = "Database Error: {}", _0)]
    DatabaseError(String),

    #[display(fmt = "Email Error: {}", _0)]
    EmailError(String),

    #[display(fmt = "Validation Error: {}", _0)]
    ValidationError(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError(_) => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            AppError::BadRequest(msg) => HttpResponse::BadRequest().json(msg),
            AppError::Unauthorized(msg) => HttpResponse::Unauthorized().json(msg),
            AppError::NotFound(msg) => HttpResponse::NotFound().json(msg),
            AppError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json("Database Error")
            }
            AppError::EmailError(msg) => HttpResponse::InternalServerError().json(msg),
            AppError::ValidationError(msg) => HttpResponse::BadRequest().json(msg),
        }
    }
}

impl From<MongoError> for AppError {
    fn from(error: MongoError) -> AppError {
        AppError::DatabaseError(error.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::InternalServerError(error.to_string())
    }
}