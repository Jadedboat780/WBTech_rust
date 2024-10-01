use crate::models::Event;
use chrono::NaiveDateTime;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub mod api_response;
pub mod endpoints;
pub mod middleware;
pub mod models;
pub mod services;

#[derive(Debug)]
pub struct EventState {
    events: RwLock<Vec<models::Event>>,
}

impl EventState {
    pub const fn new() -> Self {
        EventState {
            events: RwLock::new(Vec::new()),
        }
    }

    pub fn len(&self) -> u32 {
        self.events.read().unwrap().len() as u32
    }

    pub fn push(&self, event: Event) {
        self.events.write().unwrap().push(event)
    }

    pub fn get(&self) -> RwLockReadGuard<Vec<Event>> {
        self.events.read().unwrap()
    }

    pub fn get_mut(&self) -> RwLockWriteGuard<Vec<Event>> {
        self.events.write().unwrap()
    }

    pub fn delete(&self, id: u32) -> Result<(), ()> {
        let mut events = self.events.write().unwrap();
        if let Some(index) = events.iter().position(|e| e.id == id) {
            events.remove(index);
            return Ok(());
        };

        Err(())
    }

    pub fn filter(&self, user_id: u32, start: NaiveDateTime, end: NaiveDateTime) -> Vec<Event> {
        self.get()
            .iter()
            .filter(|e| {
                e.user_id == user_id
                    && e.date.and_hms_opt(0, 0, 0).unwrap() >= start
                    && e.date.and_hms_opt(23, 59, 59).unwrap() <= end
            })
            .cloned()
            .collect()
    }
}
