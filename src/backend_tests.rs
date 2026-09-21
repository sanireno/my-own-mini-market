use super::{AppState, build_app};
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde_json::{Value, json};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

const SECRET: &str = "test-secret-not-used-by-the-application";

fn app() -> axum::Router {
    // These requests must be rejected before any database access.
    let pool = PgPoolOptions::new()
        .connect_lazy("postgres://localhost/unused_backend_tests")
        .unwrap();
    build_app(
        AppState {
            pool,
            jwt_secret: SECRET.to_string(),
        },
        None,
    )
}

async fn request(method: &str, uri: &str, token: Option<&str>, body: Value) -> (StatusCode, Value) {
    let mut request = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json");
    if let Some(token) = token {
        request = request.header("authorization", format!("Bearer {token}"));
    }
    let response = app()
        .oneshot(request.body(Body::from(body.to_string())).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 4096).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

#[tokio::test]
async fn me_rejects_missing_invalid_expired_and_wrongly_signed_tokens() {
    use crate::user::auth::{Claims, create_token};
    let expired = encode(
        &Header::default(),
        &Claims {
            sub: "1".to_string(),
            role: "customer".to_string(),
            exp: 1,
        },
        &EncodingKey::from_secret(SECRET.as_bytes()),
    )
    .unwrap();
    let wrong_signature = create_token("1", "customer", "wrong-secret");
    for token in [
        None,
        Some("not-a-jwt"),
        Some(expired.as_str()),
        Some(wrong_signature.as_str()),
    ] {
        let (status, body) = request("GET", "/auth/me", token, Value::Null).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(body["error"], "Unauthorized");
    }
}

#[tokio::test]
async fn invalid_pagination_returns_400_before_database_access() {
    for query in [
        "page=0",
        "limit=0",
        "limit=101",
        "page=4294967296",
        "page=-1",
        "limit=abc",
    ] {
        let (status, _) = request("GET", &format!("/products?{query}"), None, Value::Null).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{query}");
    }
}

#[tokio::test]
async fn invalid_registration_returns_422_before_database_access() {
    for body in [
        json!({"username":" ", "email":"user@example.com", "password":"12345678"}),
        json!({"username":"User", "email":"invalid", "password":"12345678"}),
        json!({"username":"User", "email":"user@example.com", "password":"1234567"}),
    ] {
        let (status, body) = request("POST", "/auth/register", None, body).await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
        assert!(body["error"].is_string());
    }
}
