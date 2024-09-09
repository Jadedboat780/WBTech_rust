use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json
};
use tokio_postgres::GenericClient;

use std::sync::Arc;

use crate::api_response::{ApiError, ApiResult};
use crate::models::{CreateOrder, GetOrder};
use crate::queries::{insert_order, select_order_by_id};
use crate::AppState;

/// Приветственное сообщение
pub async fn hello_word() -> Json<String> {
    Json("Hello, World!".to_string())
}

/// Обработка ошибки 404
pub async fn handler_404() -> ApiError {
    ApiError::NotFound("Page not found".to_owned())
}

/// Получение заказа по трек-номере
pub async fn get_order(
    Path(track_number): Path<String>,
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<GetOrder>> {
    let order = select_order_by_id(track_number, state.client.client())
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(Json(order))
}

/// Создание нового заказа
pub async fn create_order(
    State(state): State<Arc<AppState>>,
    Json(order): Json<CreateOrder>,
) -> ApiResult<StatusCode> {
    insert_order(order, state.client.client())
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    Ok(StatusCode::CREATED)
}
