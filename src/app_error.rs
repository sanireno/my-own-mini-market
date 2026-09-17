use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    InternalServerError,
    NotFound,
    Conflict,
    ConflictMessage(String),
    Validation(String),
    Unauthorized,
    Forbidden,
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => AppError::NotFound,
            sqlx::Error::Database(database_error) => match database_error.code().as_deref() {
                Some("23505") => AppError::Conflict, //23505 — нарушение уникальности
                Some("23503") => {
                    //23503 — нарушение внешнего ключа
                    AppError::Validation("Referenced resource does not exist".to_string())
                }
                Some("23514") => {
                    //23514 — нарушение CHECK
                    AppError::Validation("Data violates database constraints".to_string())
                }
                Some("23502") => AppError::Validation("Required field is missing".to_string()), //23502 — нарушение NOT NULL
                _ => {
                    eprintln!("DATABASE ERROR: {database_error}");
                    AppError::InternalServerError
                }
            },
            error => {
                eprintln!("DATABASE ERROR: {error}");
                AppError::InternalServerError
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
            AppError::Conflict => (StatusCode::CONFLICT, "Conflict".to_string()),
            AppError::ConflictMessage(message) => (StatusCode::CONFLICT, message),
            AppError::Validation(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
        };
        let body = Json(json!({
            "error": message
        }));
        (status, body).into_response()
    }
}
