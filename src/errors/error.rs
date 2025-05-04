use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use mongodb::error::Error as MongoError;
use serde::Serialize;
use serde_json::json;

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

    #[display(fmt = "Forbidden: {}", _0)]
    Forbidden(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError(_) => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            AppError::BadRequest(msg) => HttpResponse::BadRequest().json(json!({
                "error": "Bad Request",
                "message": msg
            })),
            AppError::Unauthorized(msg) => HttpResponse::Unauthorized().json(json!({
                "error": "Unauthorized",
                "message": msg
            })),
            AppError::NotFound(msg) => HttpResponse::NotFound().json(json!({
                "error": "Not Found",
                "message": msg
            })),
            AppError::DatabaseError(msg) => HttpResponse::InternalServerError().json(json!({
                "error": "Database Error",
                "message": msg
            })),
            AppError::EmailError(msg) => HttpResponse::InternalServerError().json(msg),
            AppError::ValidationError(msg) => HttpResponse::BadRequest().json(json!({
                "error": "Validation Error",
                "message": msg
            })),
            AppError::Forbidden(msg) => HttpResponse::Forbidden().json(json!({
                "error": "Forbidden",
                "message": msg
            })),
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

