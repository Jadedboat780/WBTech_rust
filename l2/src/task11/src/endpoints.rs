use crate::api_response::{ApiError, ApiResponse};
use crate::models::{EventRequest, EventUpdateRequest, GetEvent};
use crate::{services, EventState};
use axum::extract::State;
use axum::{extract::Path, Json};
use serde_json::{json, Value};
use std::sync::Arc;
use axum::http::StatusCode;

pub async fn create_event(
    State(events_state): State<Arc<EventState>>,
    Json(new_event): Json<EventRequest>,
) -> ApiResponse<StatusCode> {
    services::create_event(new_event, events_state.clone());
    Ok(StatusCode::CREATED)
}

pub async fn update_event(
    State(events_state): State<Arc<EventState>>,
    Json(update_event): Json<EventUpdateRequest>,
) -> ApiResponse<StatusCode> {
    services::update_event(update_event, events_state.clone())
        .map_err(|_| ApiError::ServiceUnavailable("Event not found".into()))?;

    Ok(StatusCode::CREATED)
}

pub async fn delete_event(
    State(events_state): State<Arc<EventState>>,
    Path(id): Path<u32>,
) -> ApiResponse<StatusCode> {
    services::delete_event(id, events_state.clone())
        .map_err(|_| ApiError::ServiceUnavailable("Event not found".into()))?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn events_for_day(
    State(events_state): State<Arc<EventState>>,
    Json(params): Json<GetEvent>,
) -> ApiResponse<Json<Value>> {
    let events = services::events_for_day(params.id, params.date, events_state.clone())
        .ok_or(ApiError::ServiceUnavailable("Event not found".into()))?;

    Ok(Json(json!({ "result": events })))
}

pub async fn events_for_week(
    State(events_state): State<Arc<EventState>>,
    Json(params): Json<GetEvent>,
) -> ApiResponse<Json<Value>> {
    let events = services::events_for_week(params.id, params.date, events_state.clone())
        .ok_or(ApiError::ServiceUnavailable("Events not found for the week".into()))?;

    Ok(Json(json!({ "result": events })))
}

pub async fn events_for_month(
    State(events_state): State<Arc<EventState>>,
    Json(event): Json<GetEvent>,
) -> ApiResponse<Json<Value>> {
    let events = services::events_for_month(event.id, event.date, events_state.clone())
        .ok_or(ApiError::ServiceUnavailable("Events not found for the month".into()))?;

    Ok(Json(json!({ "result": events })))
}