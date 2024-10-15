use axum::{extract::{Path, State}, response::Json};
use std::sync::{Arc};
use axum::http::StatusCode;
use crate::models::{JoinRoom, Message, SendMessage};
use crate::api_response::{ApiResponse, ApiError};
use crate::AppState;


pub async fn join_room(
    State(chat_state): State<Arc<AppState>>,
    Json(payload): Json<JoinRoom>,
) -> ApiResponse<Json<String>> {

    let room_name= chat_state
        .rooms
        .iter()
        .find(|r| r.value().name == payload.room.name)
        .map(|r| r.key().clone())
        .unwrap_or_else(|| {
            chat_state.rooms.insert(payload.room.name.clone(), payload.room.clone());
            payload.room.name
        });

    Ok(Json(format!("User '{}' joined room '{}'.", payload.user.name, room_name)))
}

pub async fn send_message(
    State(chat_state): State<Arc<AppState>>,
    Json(payload): Json<SendMessage>,
) -> ApiResponse<StatusCode> {
    let mut room = chat_state.rooms
        .get_mut(&payload.room_name)
        .ok_or(ApiError::NotFound("Room not found".into()))?;

    // Добавляем сообщение в комнату
    room.messages.push(payload.message);

    Ok(StatusCode::CREATED)
}

pub async fn get_messages(
    State(chat_state): State<Arc<AppState>>,
    Path(room_name): Path<String>,
) -> Json<Vec<Message>> {
    let messages = if let Some(room) = chat_state.rooms.get(&room_name) {
        room.messages.clone()
    } else {
        vec![]
    };

    Json(messages)
}

pub async fn leave_room(
    State(chat_state): State<Arc<AppState>>,
    Path((room_name, username)): Path<(String, String)>,
) -> ApiResponse<Json<String>> {
    let mut room = chat_state.rooms
        .get_mut(&room_name)
        .ok_or(ApiError::NotFound("Room not found".into()))?;

    room.users.retain(|user| user.name != username);
    // room.user_count.fetch_sub(1, Ordering::Release);

    Ok(Json(format!("User '{}' left the room", username)))
}
