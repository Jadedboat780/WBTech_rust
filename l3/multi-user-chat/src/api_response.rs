use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Результат запроса на API
pub type ApiResponse<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    BadRequest,
    Forbidden,
    NotFound(String),
    InternalServerError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::BadRequest => (StatusCode::BAD_REQUEST, "Bad request".to_owned()),
            Self::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_owned()),
            Self::NotFound(err) => (StatusCode::NOT_FOUND, err),
            Self::InternalServerError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
        };

        let body = Json(json!({"error": error_message}));
        (status, body.to_owned()).into_response()
    }
}
