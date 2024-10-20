mod error;
mod key;
pub mod models;

use crate::models::user::{RegisterUser, User};
use crate::AppState;
use axum::extract::State;
use axum::Json;
use error::AuthError;
use jsonwebtoken::{encode, Header};
use key::KEYS;
use models::{AuthBody, AuthPayload, Claims};
use std::sync::Arc;
use uuid::Uuid;

type TokenResult<T> = Result<T, jsonwebtoken::errors::Error>;
fn new_access_token(id: Uuid) -> TokenResult<AuthBody> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: id,
        exp: expiration,
    };

    let token = encode(&Header::default(), &claims, &KEYS.encoding)?;
    Ok(AuthBody::new(token, id))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthBody>, AuthError> {
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let client = &state.get_client().await;
    let id = User::get(client, payload.username, payload.password)
        .await
        .map_err(|_| AuthError::WrongCredentials)?;

    let auth_body = new_access_token(id).map_err(|_| AuthError::TokenCreation)?;
    Ok(Json(auth_body))
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterUser>,
) -> Result<Json<AuthBody>, AuthError> {
    if payload.name.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let client = state.get_client().await;
    let id = User::new(&client, payload.name.clone(), payload.password.clone())
        .await
        .map_err(|_| AuthError::WrongCredentials)?;

    let auth_body = new_access_token(id).map_err(|_| AuthError::TokenCreation)?;
    Ok(Json(auth_body))
}
