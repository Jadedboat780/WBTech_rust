use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

/// Результат запроса на API
pub type ApiResponse<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    /// Error 400
    BadRequest(String),
    /// Error 500
    InternalServerError(String),
    /// Error 503
    ServiceUnavailable(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::ServiceUnavailable(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg),
            ApiError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(serde_json::json!({ "error": message }));

        (status, body.to_owned()).into_response()
    }
}
