use std::fmt::Display;

use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorBody {
    status: u16,
    error: String,
    description: String,
}

#[derive(Debug)]
pub enum Error {
    MissingEnv(String),
    InvalidEnv(String),
    DatabaseError(String),
    NotFoundError(String),
    UnauthorizedError(String),
    ForbiddenError(String),
    InternalServerError(String),
    BadRequestError(String),
}

impl Error {
    pub fn from_sqlx(err: sqlx::Error, entity: &str) -> Error {
        match err {
            sqlx::Error::RowNotFound => Self::NotFoundError(format!("{} not found", entity)),
            _ => Self::DatabaseError(err.to_string()),
        }
    }

    pub fn error_message(&self) -> String {
        self.status_code()
            .canonical_reason()
            .map(|r| r.to_string())
            .unwrap_or(self.status_code().to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingEnv(val) => write!(f, "Missing environment variable: {}", val),
            Self::InvalidEnv(val) => write!(f, "Invalid environment variable: {}", val),
            Self::DatabaseError(val) => write!(f, "{}", val),
            Self::NotFoundError(val) => write!(f, "{}", val),
            Self::UnauthorizedError(val) => write!(f, "{}", val),
            Self::ForbiddenError(val) => write!(f, "{}", val),
            Self::InternalServerError(val) => write!(f, "{}", val),
            Self::BadRequestError(val) => write!(f, "{}", val),
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            Self::NotFoundError(_) => actix_web::http::StatusCode::NOT_FOUND,
            Self::ForbiddenError(_) => actix_web::http::StatusCode::FORBIDDEN,
            Self::UnauthorizedError(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            Self::BadRequestError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let body = ErrorBody {
            status: self.status_code().as_u16(),
            error: self.error_message(),
            description: self.to_string(),
        };
        HttpResponse::build(self.status_code()).json(body)
    }
}
