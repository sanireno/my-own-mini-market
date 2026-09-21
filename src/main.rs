use axum::Router;
use axum::http::{
    HeaderName, HeaderValue, Method,
    header::{AUTHORIZATION, CONTENT_TYPE},
};
use axum::routing::{get, patch, post};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};
mod category;
pub mod category_models;
mod product;
pub mod products_models;
pub mod user_models;
use cart::cart_handler::*;
use category::category_handler::*;
use product::product_handler::*;
mod app_error;
mod cart;
mod cart_models;
mod inventory;
mod order;
mod order_models;
mod user;

use order::order_handler::*;
use order::order_service::expire_pending_orders;
use user::user_handler::*;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("не установлен database");
    let jwt_secret = std::env::var("JWT_SECRET").expect("не найдено секретное слово");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("ошибка подключения к sql");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Ошибка миграций");
    println!("База подключена, миграции применены");
    spawn_expired_order_worker(pool.clone());
    let state = AppState { pool, jwt_secret };
    let frontend_origins = std::env::var("FRONTEND_ORIGIN").ok();
    let app = build_app(state, frontend_origins.as_deref());
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn build_app(state: AppState, frontend_origins: Option<&str>) -> Router {
    let cors = cors_layer(frontend_origins);
    Router::new()
        // products
        .route(
            "/products",
            get(get_all_product_handler).post(create_product_handler),
        )
        .route(
            "/products/{id}",
            get(get_product_handler)
                .put(update_product_handler)
                .delete(delete_product_handler),
        )
        // categories
        .route(
            "/categories",
            get(get_all_category_handler).post(create_category_handler),
        )
        .route(
            "/categories/{id}",
            get(get_category_handler)
                .put(update_category_handler)
                .delete(delete_category_handler),
        )
        .route(
            "/categories/{id}/products",
            get(get_products_from_category_handler),
        )
        //user
        .route("/auth/register", post(create_user_handler))
        .route("/auth/login", post(login_handler))
        .route("/auth/me", get(get_current_user_handler))
        // cart
        .route("/cart", get(get_cart_handler))
        .route("/cart/items", post(create_cart_item_handler))
        .route(
            "/cart/items/{id}",
            patch(update_cart_item_handler).delete(delete_cart_item_handler),
        )
        // checkout and orders
        .route("/checkout", post(checkout_handler))
        .route("/orders", get(list_orders_handler))
        .route("/orders/{id}", get(get_order_handler))
        .route("/orders/{id}/cancel", post(cancel_order_handler))
        .route(
            "/orders/{id}/simulate-payment",
            post(simulate_payment_handler),
        )
        .layer(cors)
        .with_state(state)
}

fn cors_layer(configured_origins: Option<&str>) -> CorsLayer {
    let configured_origins =
        configured_origins.unwrap_or("http://localhost:5173,http://127.0.0.1:5173");
    let allowed_origins = configured_origins
        .split(',')
        .map(str::trim)
        .filter(|origin| !origin.is_empty())
        .map(|origin| {
            origin
                .parse::<HeaderValue>()
                .expect("FRONTEND_ORIGIN contains an invalid origin")
        })
        .collect::<Vec<_>>();

    assert!(
        !allowed_origins.is_empty(),
        "FRONTEND_ORIGIN must contain at least one origin"
    );

    CorsLayer::new()
        .allow_origin(AllowOrigin::list(allowed_origins))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([
            AUTHORIZATION,
            CONTENT_TYPE,
            HeaderName::from_static("idempotency-key"),
        ])
        .max_age(Duration::from_secs(3600))
}

fn spawn_expired_order_worker(pool: PgPool) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        loop {
            interval.tick().await;
            if let Err(error) = expire_pending_orders(&pool).await {
                eprintln!("failed to expire pending orders: {error:?}");
            }
        }
    });
}

#[cfg(test)]
mod cors_tests {
    use super::cors_layer;
    use axum::Router;
    use axum::body::Body;
    use axum::http::{
        Method, Request, StatusCode,
        header::{ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_REQUEST_METHOD, ORIGIN},
    };
    use axum::routing::get;
    use tower::ServiceExt;

    fn test_app() -> Router {
        Router::new()
            .route("/", get(|| async {}))
            .layer(cors_layer(Some("http://localhost:5173")))
    }

    #[tokio::test]
    async fn allows_configured_origin() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .method(Method::OPTIONS)
                    .uri("/")
                    .header(ORIGIN, "http://localhost:5173")
                    .header(ACCESS_CONTROL_REQUEST_METHOD, "POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(ACCESS_CONTROL_ALLOW_ORIGIN),
            Some(&"http://localhost:5173".parse().unwrap())
        );
    }

    #[tokio::test]
    async fn rejects_unconfigured_origin() {
        let response = test_app()
            .oneshot(
                Request::builder()
                    .method(Method::OPTIONS)
                    .uri("/")
                    .header(ORIGIN, "https://untrusted.example")
                    .header(ACCESS_CONTROL_REQUEST_METHOD, "POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert!(
            response
                .headers()
                .get(ACCESS_CONTROL_ALLOW_ORIGIN)
                .is_none()
        );
    }
}

#[cfg(test)]
mod backend_tests;
