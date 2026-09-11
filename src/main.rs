use axum::routing::post;
use axum::{Router, routing::get};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
mod category;
pub mod category_models;
mod product;
pub mod products_models;
pub mod user_models;
use category::category_handler::*;
use product::product_handler::*;
mod app_error;
mod user;
mod cart_models;
mod cart;

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
    let state = AppState { pool, jwt_secret };
    let app = Router::new()
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
        .with_state(state);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
