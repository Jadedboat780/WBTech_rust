use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub date: NaiveDate,
    pub user_id: u32,
}

#[derive(Deserialize)]
pub struct EventRequest {
    pub title: String,
    pub description: Option<String>,
    pub date: NaiveDate,
    pub user_id: u32,
}

#[derive(Deserialize)]
pub struct EventUpdateRequest {
    pub id: u32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub date: Option<NaiveDate>,
}

#[derive(Deserialize)]
pub struct GetEvent {
    pub id: u32,
    pub date: NaiveDate,
}
