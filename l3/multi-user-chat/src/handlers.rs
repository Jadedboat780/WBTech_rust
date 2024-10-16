use crate::api_response::{ApiError, ApiResponse};
use crate::models::{JoinRoom, LeaveRoom, Message, Room, SendMessage, User};
use crate::{AppState, RoomName, UserId};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Создание новой комнаты
pub async fn create_room(
    State(chat_state): State<Arc<AppState>>,
    Path(room_name): Path<RoomName>,
) -> (StatusCode, String) {
    chat_state
        .rooms
        .insert(room_name.clone(), RwLock::new(Room::new()));
    (
        StatusCode::CREATED,
        format!("Cretate room with name {}", room_name),
    )
}

/// Создание новой комнаты
pub async fn create_user(
    State(chat_state): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> (StatusCode, String) {
    let user_count = chat_state.users.len() as UserId;
    chat_state
        .users
        .insert(user_count, RwLock::new(User::new(username)));
    (StatusCode::CREATED, user_count.to_string())
}

/// Добавление пользователя в комнату
pub async fn join_room(
    State(chat_state): State<Arc<AppState>>,
    Json(payload): Json<JoinRoom>,
) -> ApiResponse<Json<String>> {
    // Добавление юзера в комнату
    chat_state
        .get_mut_room(&payload.room_name)
        .ok_or(ApiError::NotFound("Room not found".into()))?
        .write()
        .await
        .add_user(payload.user_id);

    Ok(Json(format!(
        "User with id {} joined room '{}'.",
        payload.user_id, payload.room_name
    )))
}

/// Отправка сообщения в чат
pub async fn send_message(
    State(chat_state): State<Arc<AppState>>,
    Json(payload): Json<SendMessage>,
) -> ApiResponse<StatusCode> {
    // Проверка, находится ли пользователь в чате
    if !chat_state.users.contains_key(&payload.message.user_id) {
        return Err(ApiError::NotFound("The user did not join this chat".into()));
    }

    // Добавление сообщения в комнату
    chat_state
        .get_mut_room(&payload.room_name)
        .ok_or(ApiError::NotFound("Room not found".into()))?
        .write()
        .await
        .messages
        .push(payload.message);

    Ok(StatusCode::CREATED)
}

/// Получить список сообщений
pub async fn get_messages(
    State(chat_state): State<Arc<AppState>>,
    Path(room_name): Path<RoomName>,
) -> ApiResponse<Json<Vec<Message>>> {
    // Получение списка сообщений комнаты
    let messages = chat_state
        .get_room(&room_name)
        .ok_or(ApiError::NotFound("Room not found".into()))?
        .read()
        .await
        .messages
        .clone();

    Ok(Json(messages))
}

/// Выход пользователя из комнаты из комнаты
pub async fn leave_room(
    State(chat_state): State<Arc<AppState>>,
    Path(payload): Path<LeaveRoom>,
) -> ApiResponse<Json<String>> {
    // Удаление юзера из комнаты
    chat_state
        .get_mut_room(&payload.room_name)
        .ok_or(ApiError::NotFound("Room not found".into()))?
        .write()
        .await
        .remove_user(payload.user_id);

    Ok(Json(format!(
        "User with id {} left the room",
        payload.user_id
    )))
}
