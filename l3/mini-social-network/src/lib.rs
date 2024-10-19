pub mod api_response;
pub mod auth;
pub mod handlers;
pub mod models;

use axum::{routing, Router};
use deadpool_postgres::{Config, Pool, Runtime::Tokio1};
use handlers::{create_post, delete_post, get_post, like_post, login_user, register_user};
use std::sync::Arc;
use tokio::net::TcpListener;

type Client = deadpool_postgres::Object;
struct AppState {
    pool: Pool,
}

impl AppState {
    pub async fn get_client(&self) -> Client {
        self.pool.get().await.unwrap()
    }
}

/// Инициализация подключения к базе данных
fn init_db_connect() -> Pool {
    let mut config = Config::new();

    config.user = std::env::var("DB_USER").ok();
    config.password = std::env::var("DB_USER_PASSWORD").ok();
    config.host = std::env::var("DB_HOST").ok();
    config.port = std::env::var("DB_PORT").unwrap().parse::<u16>().ok();
    config.dbname = std::env::var("DB_NAME").ok();

    config.create_pool(Some(Tokio1), tokio_postgres::NoTls).unwrap()
}

pub async fn init_router() -> Router {
    let pool = init_db_connect();
    let state = Arc::new(AppState { pool });
    Router::new()
        .route("/register", routing::post(register_user))
        .route("/login", routing::post(login_user))
        .route("/posts", routing::post(create_post))
        .route(
            "/posts/:post_id",
            routing::get(get_post).delete(delete_post),
        )
        .route("/posts/:post_id/likes", routing::post(like_post))
        .with_state(state)
}

/// Инициализация TCP слушателя
pub async fn init_tcp_listener() -> TcpListener {
    let host = std::env::var("HOST").expect("Хост не установлен");
    let port = std::env::var("PORT").expect("Порт не установлен");
    let addr = format!("{}:{}", host, port);

    TcpListener::bind(addr).await.unwrap()
}
