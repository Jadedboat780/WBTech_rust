pub mod handlers;
pub mod models;
pub mod api_response;

use dashmap::DashMap;
use models::Room;


#[derive(Debug)]
pub struct AppState {
    pub rooms: DashMap<String, Room>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            rooms: DashMap::new(),
        }
    }
}
