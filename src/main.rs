use axum::{routing::get, Router};
use axum::routing::post;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
pub mod products_models;
pub mod user_models;
pub mod category_models;
mod category;
mod product;
use category::category_handler::*;
use product::product_handler::*;
mod user;
use user::user_handler::*;
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
            get(get_all_product_handler)
                .post(create_product_handler),
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
            get(get_all_category_handler)
                .post(create_category_handler),
        )
        .route(
            "/categories/{id}",
            get(get_category_handler)
                .put(update_category_handler)
                .delete(delete_category_handler),
        )
        .route("/categories/{id}/products",
               get(get_products_from_category_handler)
        )
        //user
        .route("/auth/register",post(create_user_handler))
        .route("/auth/login",post(login_handler))

        .with_state(pool);
    let listener=TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener,app)
        .await.unwrap();
}
