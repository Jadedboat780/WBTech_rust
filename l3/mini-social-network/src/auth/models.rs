use super::error::AuthError;
use super::key::KEYS;
use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

#[axum::async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Claims {
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct AuthBody {
    access_token: String,
    token_type: String,
    user_id: Uuid,
}

impl AuthBody {
    pub fn new(access_token: String, user_id: Uuid) -> Self {
        Self {
            access_token,
            token_type: "Bearer".into(),
            user_id,
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct AuthPayload {
    pub username: String,
    pub password: String,
}
