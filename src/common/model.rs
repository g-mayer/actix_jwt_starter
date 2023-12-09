use actix_web::{error::ResponseError, HttpResponse};
use serde::Serialize;
use utoipa::{ToResponse, ToSchema};

#[derive(Debug, Serialize, ToSchema, ToResponse)]
pub enum AppError {
    DatabaseError(String),
    ValidationError(String),
    NotFoundError(String),
    UnauthorizedError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DatabaseError(message) => HttpResponse::InternalServerError().json(message),
            AppError::ValidationError(message) => HttpResponse::BadRequest().json(message),
            AppError::NotFoundError(message) => HttpResponse::NotFound().json(message),
            AppError::UnauthorizedError(message) => HttpResponse::Unauthorized().json(message),
        }
    }
}
