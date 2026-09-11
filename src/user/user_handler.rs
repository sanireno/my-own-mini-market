use crate::AppState;
use crate::app_error::AppError;
use crate::user::auth::*;
use crate::user::password::verify_password;
use crate::user::user_repository::{create_user, find_user_by_email};
use crate::user_models::{LoginResponse, LoginUser, RegisterUser, UserResponse};
use axum::Json;
use axum::extract::State;
pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(user): Json<RegisterUser>,
) -> Result<Json<UserResponse>, AppError> {
    let user = create_user(&state.pool, user).await?;
    let user_response = UserResponse {
        id: user.id,
        username: user.username,
        email: user.email,
        created_at: user.created_at,
        role: user.role,
    };
    Ok(Json(user_response))
}
pub async fn login_handler(
    State(state): State<AppState>,
    Json(login): Json<LoginUser>,
) -> Result<Json<LoginResponse>, AppError> {
    let user = find_user_by_email(&state.pool, login.email)
        .await
        .map_err(|error| match error {
            sqlx::Error::RowNotFound => AppError::Unauthorized,
            error => AppError::from(error),
        })?;
    let response = verify_password(&login.password, &user.password_hash);
    match response {
        Ok(true) => {
            println!("success password");
            let token = create_token(user.id.to_string().as_str(), &user.role, &state.jwt_secret);
            Ok(Json(LoginResponse { token }))
        }
        Ok(false) => Err(AppError::Unauthorized),
        Err(error) => {
            eprintln!("PASSWORD VERIFICATION ERROR: {error}");
            Err(AppError::InternalServerError)
        }
    }
}
