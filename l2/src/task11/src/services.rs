use crate::models::{Event, EventRequest, EventUpdateRequest};
use crate::EventState;
use chrono::{Datelike, Duration, NaiveDate};
use std::sync::Arc;

pub fn create_event(new_event: EventRequest, events_state: Arc<EventState>) {
    let new_event = Event {
        id: events_state.len(),
        title: new_event.title,
        description: new_event.description,
        date: new_event.date,
        user_id: new_event.user_id,
    };

    events_state.push(new_event);
}

pub fn update_event(event: EventUpdateRequest, events_state: Arc<EventState>) -> Result<(), ()> {
    if let Some(e) = events_state.get_mut().iter_mut().find(|e| e.id == event.id) {
        if let Some(title) = event.title {
            e.title = title;
        }
        if let Some(desc) = event.description {
            e.description = Some(desc);
        }
        if let Some(date) = event.date {
            e.date = date;
        }
        return Ok(());
    }

    Err(())
}

pub fn delete_event(id: u32, events_state: Arc<EventState>) -> Result<(), ()> {
    events_state.delete(id).map_err(|_| ())?;
    Ok(())
}

pub fn events_for_day(
    user_id: u32,
    date: NaiveDate,
    events_state: Arc<EventState>,
) -> Option<Vec<Event>> {
    let start_of_day = date.and_hms_opt(0, 0, 0).unwrap();
    let end_of_day = date.and_hms_opt(23, 59, 59).unwrap();
    let events = events_state.filter(user_id, start_of_day, end_of_day);

    if events.is_empty() {
        None
    } else {
        Some(events)
    }
}

pub fn events_for_week(
    user_id: u32,
    date: NaiveDate,
    events_state: Arc<EventState>,
) -> Option<Vec<Event>> {
    let start_of_week = date.and_hms_opt(0, 0, 0).unwrap();
    let end_of_week = start_of_week + Duration::days(6);

    let events = events_state.filter(user_id, start_of_week, end_of_week);

    if events.is_empty() {
        None
    } else {
        Some(events)
    }
}

pub fn events_for_month(
    user_id: u32,
    date: NaiveDate,
    events_state: Arc<EventState>,
) -> Option<Vec<Event>> {
    let start_of_month = date.with_day(1).unwrap().and_hms_opt(0, 0, 0).unwrap();

    let last_day_of_month = date
        .with_day(1)
        .unwrap()
        .with_month(date.month() + 1)
        .unwrap_or_else(|| date.with_month(1).unwrap())
        .pred_opt()
        .unwrap()
        .and_hms_opt(23, 59, 59)
        .unwrap();

    let events = events_state.filter(user_id, start_of_month, last_day_of_month);

    if events.is_empty() {
        None
    } else {
        Some(events)
    }
}
