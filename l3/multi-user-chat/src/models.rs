use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub content: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub name: String,
    pub messages: Vec<Message>,
    pub users: Vec<User>
}

#[derive(Deserialize)]
pub struct JoinRoom {
    pub room: Room,
    pub user: User,
}

#[derive(Deserialize)]
pub struct SendMessage {
    pub room_name: String,
    pub username: String,
    pub message: Message,
}

