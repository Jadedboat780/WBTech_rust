pub mod api_response;
pub mod handlers;
pub mod models;

use crate::models::User;
use dashmap::mapref::one::{Ref, RefMut};
use dashmap::DashMap;
use models::Room;
use tokio::sync::RwLock;

type RoomName = String;
type UserId = u32;

#[derive(Debug)]
pub struct AppState {
    pub rooms: DashMap<RoomName, RwLock<Room>>,
    pub users: DashMap<UserId, RwLock<User>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            rooms: DashMap::new(),
            users: DashMap::new(),
        }
    }
    pub fn get_room(&self, room_name: &str) -> Option<Ref<RoomName, RwLock<Room>>> {
        self.rooms.get(room_name)
    }

    pub fn get_mut_room(&self, room_name: &str) -> Option<RefMut<RoomName, RwLock<Room>>> {
        self.rooms.get_mut(room_name)
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
