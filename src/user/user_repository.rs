use sqlx::PgPool;
use crate::user::password::*;
use crate::user_models::*;
pub async fn create_user(pool:&PgPool,user:RegisterUser)->Result<User,sqlx::Error>{
    let hashed_password=hash_password(user.password.as_str()).unwrap();
    sqlx::query_as!(
       User,
     r#"
        INSERT INTO users (username,email,password_hash)
        VALUES ($1,$2,$3)
        RETURNING id,username,email,password_hash,created_at
"#,
        user.username,
        user.email,
        hashed_password
    )
        .fetch_one(pool)
        .await
}
pub async fn find_user_by_email(pool:&PgPool,email:String)->Result<User,sqlx::Error>{
    sqlx::query_as!(
        User,
        r#"
        SELECT id,username,email,password_hash,created_at
        FROM users
        WHERE email=$1
        "#,
        email
    )
        .fetch_one(pool)
        .await
}