use crate::models::{NewTask, Task};
use crate::notifications::{notify_users, TaskNotify};
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use redis::AsyncCommands;
use serde_json::{json, Value};
use std::sync::Arc;

type ApiResponse<T> = Result<T, StatusCode>;

pub async fn create_task(
    State(state): State<Arc<AppState>>,
    Json(new_task): Json<NewTask>,
) -> ApiResponse<Json<Task>> {
    let task = Task::create(&state.db_pool, new_task)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    notify_users(&state.redis_client, TaskNotify::CREATED, task.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(task))
}

pub async fn get_task(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> ApiResponse<Json<Task>> {
    let task = Task::get(&state.db_pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(task))
}

pub async fn complete_task(
    Path(id): Path<i32>,
    State(state): State<Arc<AppState>>,
) -> ApiResponse<Json<Value>> {
    Task::complete(&state.db_pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    notify_users(&state.redis_client, TaskNotify::COMPLETED, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({"status": "completed"})))
}

pub async fn get_notifications(
    State(state): State<Arc<AppState>>,
) -> ApiResponse<Json<Vec<String>>> {
    let mut con = state.redis_client.lock().await;

    let notifications: Vec<String> = con
        .lrange("task_notifications_list", 0, -1)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(notifications))
}
