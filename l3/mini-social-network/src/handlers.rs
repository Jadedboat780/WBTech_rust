use crate::api_response::{ApiError, ApiResult};
use crate::models::{
    post::{CreatePost, Post},
    user::{LoginUser, RegisterUser, User},
};
use crate::AppState;
use axum::{extract::Path, extract::State, http::StatusCode, Json};
use std::sync::Arc;
use uuid::Uuid;

pub async fn register_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterUser>,
) -> ApiResult<Json<User>> {
    let client = state.get_client().await;

    let id = User::new(&client, payload.name.clone(), payload.password.clone())
        .await
        .map_err(|_| ApiError::InternalServerError("Interval server error".into()))?;

    let user = User {
        id,
        name: payload.name,
        password: payload.password,
    };
    Ok(Json(user))
}

pub async fn login_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginUser>,
) -> ApiResult<String> {
    let client = state.get_client().await;

    let id = User::get(&client, payload.name, payload.password)
        .await
        .map_err(|_| ApiError::NotFound("User not found".into()))?;

    Ok(id.to_string())
}

pub async fn create_post(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePost>,
) -> ApiResult<StatusCode> {
    let client = state.get_client().await;

    let user_id = User::get(
        &client,
        payload.login_user.name,
        payload.login_user.password,
    )
    .await
    .map_err(|_| ApiError::NotFound("User not found".into()))?;

    Post::new(&client, user_id, payload.content)
        .await
        .map_err(|_| ApiError::InternalServerError("Interval server error".into()))?;

    Ok(StatusCode::CREATED)
}

pub async fn get_post(
    Path(post_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> ApiResult<Json<Post>> {
    let client = state.get_client().await;

    let post = Post::get(&client, post_id)
        .await
        .map_err(|_| ApiError::NotFound("Post not found".into()))?;

    Ok(Json(post))
}

pub async fn delete_post(
    Path(post_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> ApiResult<StatusCode> {
    let client = state.get_client().await;

    Post::delete(&client, post_id)
        .await
        .map_err(|_| ApiError::InternalServerError("Interval server error".into()))?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn like_post(
    Path(post_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> ApiResult<StatusCode> {
    let client = state.get_client().await;

    Post::add_like(&client, post_id)
        .await
        .map_err(|_| ApiError::InternalServerError("Interval server error".into()))?;

    Ok(StatusCode::OK)
}
