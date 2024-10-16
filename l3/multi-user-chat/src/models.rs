use crate::{RoomName, UserId};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Пользователь
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
}

/// Сообщение
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub user_id: UserId,
    pub content: String,
}

/// Комната
#[derive(Debug, Clone, Default)]
pub struct Room {
    pub messages: Vec<Message>,
    users: Vec<UserId>,
    users_count: Arc<AtomicUsize>,
}

#[derive(Deserialize)]
pub struct JoinRoom {
    pub room_name: RoomName,
    pub user_id: UserId,
}

#[derive(Deserialize)]
pub struct LeaveRoom {
    pub room_name: RoomName,
    pub user_id: UserId,
}

#[derive(Deserialize)]
pub struct SendMessage {
    pub room_name: RoomName,
    pub message: Message,
}

impl User {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
impl Room {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Добавление юзера в комнату
    pub fn add_user(&mut self, id: u32) {
        // Добавление id пользователя в список
        self.users.push(id);
        // Увеличение счётчика юзеров
        self.users_count.fetch_add(1, Ordering::Release);
    }

    /// Удаление юзера из комнаты
    pub fn remove_user(&mut self, id: UserId) {
        if let Some(pos) = self.users.iter().position(|&user_id| user_id == id) {
            // Удаление id пользователя из списка
            self.users.remove(pos);
            // Уменьшение счётчика юзеров
            self.users_count.fetch_sub(1, Ordering::Release);
        }
    }

    /// Получение количества юзеров в комнате
    pub fn users_count(&self) -> usize {
        self.users_count.load(Ordering::Relaxed)
    }
}
