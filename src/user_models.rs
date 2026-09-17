use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub role: String,
}
#[derive(Serialize, Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
}
#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
#[derive(Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub role: String,
}
#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}
