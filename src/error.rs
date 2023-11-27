use anyhow::{anyhow, Error};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

impl From<AuthError> for anyhow::Error {
    fn from(value: AuthError) -> Self {
        match value {
            AuthError::WrongCredentials => Error::msg("Wrong credentials"),
            AuthError::MissingCredentials => Error::msg("Missing credentials"),
            AuthError::TokenCreation => Error::msg("Token creation error"),
            AuthError::InvalidToken => Error::msg("Invalid token"),
        }
    }
}

pub struct AppError {
    status_code: StatusCode,
    code: String,
    error: anyhow::Error,
}

impl AppError {
    pub fn new<E>(status_code: StatusCode, code: &str, error: E) -> Self
    where
        E: Into<anyhow::Error>,
    {
        let status_code = status_code.clone();
        let error = error.into();
        let code = code.to_string();
        Self {
            status_code,
            code,
            error,
        }
    }
}

impl Default for AppError {
    fn default() -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            code: "unknown_error".to_string(),
            error: anyhow!("Unknown error"),
        }
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            code: "unknown_error".to_string(),
            error: err.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.status_code,
            Json(json!({
                "code": self.code,
                "detail": format!("Something went wrong: {}", self.error)
            })),
        )
            .into_response()
    }
}
