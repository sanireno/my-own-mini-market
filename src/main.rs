use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

pub mod products_models;
mod repository;
mod handler;
mod category_models;

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
    let app = Router::new()
        // products
        .route(
            "/products",
            get(handler::get_all_product_handler)
                .post(handler::create_product_handler),
        )
        .route(
            "/products/{id}",
            get(handler::get_product_handler)
                .put(handler::update_product_handler)
                .delete(handler::delete_product_handler),
        )

        // categories
        .route(
            "/categories",
            get(handler::get_all_category_handler)
                .post(handler::create_category_handler),
        )
        .route(
            "/categories/{id}",
            get(handler::get_category_handler)
                .put(handler::update_category_handler)
                .delete(handler::delete_category_handler),
        )
        .route("/categories/{id}/products",
               get(handler::get_products_from_category_handler)
        )

        .with_state(pool);
    let listener=TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener,app)
        .await.unwrap();
}
