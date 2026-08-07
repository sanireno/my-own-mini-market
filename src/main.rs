use axum::routing::{delete, post, put};
use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

pub mod products_models;
mod repository;
mod handler;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url=std::env::var("DATABASE_URL").expect("не установлен database");
    let pool=PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("ошибка подключения к sql");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Ошибка миграций");
    println!("База подключена, миграции применены");
    let app=Router::new()
        .route("/products",post(handler::create_product_handler))
        .route("/products/:id",get(handler::get_product_handler))
        .route("/products/:id",delete(handler::delete_product_handler))
        .route("/products",get(handler::get_all_product_handler))
        .route("/products/:id",put(handler::update_product_handler))
        .with_state(pool);
    let listener=TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener,app)
        .await.unwrap();
}
