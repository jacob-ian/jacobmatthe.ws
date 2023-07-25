use std::fmt::Display;

use actix_web::{body::BoxBody, http, ResponseError};

use crate::{
    components::{article, error},
    html::HtmlResponse,
};

#[derive(Debug, Clone)]
pub enum Error {
    Internal(String),
    BadRequest(String),
    NotFound(String),
    Unauthorized(String),
    Forbidden(String),
}

impl Error {
    pub fn title(&self) -> &'static str {
        return self.status_code().canonical_reason().unwrap_or("Error");
    }

    pub fn body(&self) -> String {
        let content = match self {
            Self::NotFound(_) => String::from(
                r#"<p>Sorry, I couldn't find that page.</p><p>Are you sure you have the right address?</p>"#,
            ),
            _ => String::from(
                r#"<p>Sorry, something went wrong.</p><p>If you've seen this a few times, feel free to send me an <a class="text-sky-400 font-bold" href="mailto:jacob@jacobmatthe.ws">email</a>.</p>"#,
            ),
        };
        return error::render(self.title(), content);
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Internal(m) => write!(f, "Internal error: {}", m),
            Self::BadRequest(m) => write!(f, "Bad request: {}", m),
            Self::NotFound(m) => write!(f, "Not found: {}", m),
            Self::Unauthorized(m) => write!(f, "Unauthorized: {}", m),
            Self::Forbidden(m) => write!(f, "Forbidden: {}", m),
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> http::StatusCode {
        match self {
            Self::Internal(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound(_) => http::StatusCode::NOT_FOUND,
            Self::BadRequest(_) => http::StatusCode::BAD_REQUEST,
            Self::Forbidden(_) => http::StatusCode::FORBIDDEN,
            Self::Unauthorized(_) => http::StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<BoxBody> {
        return HtmlResponse::builder()
            .title(format!("{} | Jacob Matthews", self.title()))
            .status(self.status_code())
            .body(self.body())
            .build()
            .into();
    }
}
