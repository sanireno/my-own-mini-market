use crate::AppState;
use crate::app_error::AppError;
use axum::extract::FromRequestParts;
use axum::http::{header::AUTHORIZATION, request::Parts};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub role: String,
    pub exp: usize, // expiration timestamp
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Admin,
    Customer,
}

pub struct AuthUser {
    pub id: i64,
    pub role: Role,
}

impl AuthUser {
    pub fn require_admin(&self) -> Result<(), AppError> {
        if self.role == Role::Admin {
            Ok(())
        } else {
            Err(AppError::Forbidden)
        }
    }
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .ok_or(AppError::Unauthorized)?;
        let claims = verify_token(token, &state.jwt_secret).map_err(|_| AppError::Unauthorized)?;
        let id = claims
            .sub
            .parse::<i64>()
            .map_err(|_| AppError::Unauthorized)?;
        let role = match claims.role.as_str() {
            "admin" => Role::Admin,
            "customer" => Role::Customer,
            _ => return Err(AppError::Unauthorized),
        };

        Ok(Self { id, role })
    }
}

pub fn create_token(user_id: &str, role: &str, secret: &str) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}
