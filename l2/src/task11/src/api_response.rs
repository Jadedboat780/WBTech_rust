use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use tower::{timeout, BoxError};

/// Результат запроса на API
pub type ApiResponse<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    /// Error 400
    BadRequest(String),
    RequestTimeout,
    /// Error 500
    InternalServerError(String),
    /// Error 503
    ServiceUnavailable(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::RequestTimeout => (StatusCode::REQUEST_TIMEOUT, "Request timeout".to_owned()),
            Self::ServiceUnavailable(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg),
            Self::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(serde_json::json!({ "error": message }));

        (status, body.to_owned()).into_response()
    }
}

pub async fn handle_timeout_error(err: BoxError) -> ApiError {
    if err.is::<timeout::error::Elapsed>() {
        ApiError::RequestTimeout
    } else {
        ApiError::InternalServerError(err.to_string())
    }
}
