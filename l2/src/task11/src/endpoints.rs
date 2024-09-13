use crate::api_response::{ApiError, ApiResult};

/// Обработка ошибки 404
pub async fn handler_404() -> ApiError {
    ApiError::NotFound("Page not found".to_owned())
}

pub async fn create_event() -> ApiResult<()> {
    todo!()
}

pub async fn update_event() -> ApiResult<()> {
    todo!()
}

pub async fn delete_event() -> ApiResult<()> {
    todo!()
}

pub async fn events_for_day() -> ApiResult<()> {
    todo!()
}

pub async fn events_for_week() -> ApiResult<()> {
    todo!()
}

pub async fn events_for_month() -> ApiResult<()> {
    todo!()
}
