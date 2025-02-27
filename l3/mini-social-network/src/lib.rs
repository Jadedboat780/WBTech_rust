pub mod api_response;
pub mod auth;
pub mod handlers;
pub mod models;

use crate::api_response::handle_timeout_error;
use auth::{login, register};
use axum::error_handling::HandleErrorLayer;
use axum::{routing, Router};
use deadpool_postgres::{Config, Pool, Runtime::Tokio1};
use handlers::{create_post, delete_post, get_post, like_post};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tower::timeout::TimeoutLayer;
use tower::ServiceBuilder;

type Client = deadpool_postgres::Object;

/// Приложение
struct AppState {
    pool: Pool,
}

impl AppState {
    /// Получить соединение с БД
    pub async fn get_client(&self) -> Client {
        self.pool.get().await.unwrap()
    }
}

/// Приложение
pub struct App {
    listener: TcpListener,
    router: Router,
}

impl App {
    /// Конструктор
    pub async fn new() -> Self {
        let listener = Self::init_tcp_listener().await;
        let router = Self::init_router().await;

        App { listener, router }
    }

    /// Инициализация подключения к базе данных
    fn init_db_connect() -> Pool {
        let mut config = Config::new();

        config.user = std::env::var("DB_USER").ok();
        config.password = std::env::var("DB_USER_PASSWORD").ok();
        config.host = std::env::var("DB_HOST").ok();
        config.port = std::env::var("DB_PORT").unwrap().parse::<u16>().ok();
        config.dbname = std::env::var("DB_NAME").ok();

        config
            .create_pool(Some(Tokio1), tokio_postgres::NoTls)
            .unwrap()
    }

    /// Инициализация роутера
    async fn init_router() -> Router {
        let pool = Self::init_db_connect();
        let state = Arc::new(AppState { pool });
        let service = ServiceBuilder::new()
            .layer(HandleErrorLayer::new(handle_timeout_error))
            .layer(TimeoutLayer::new(Duration::from_secs(10)));

        Router::new()
            .route("/register", routing::post(register))
            .route("/login", routing::post(login))
            .route("/posts", routing::post(create_post))
            .route(
                "/posts/:post_id",
                routing::get(get_post).delete(delete_post),
            )
            .route("/posts/:post_id/likes", routing::post(like_post))
            .with_state(state)
            .layer(service)
    }

    /// Инициализация TCP слушателя
    async fn init_tcp_listener() -> TcpListener {
        let host = std::env::var("HOST").expect("Хост не установлен");
        let port = std::env::var("PORT").expect("Порт не установлен");
        let addr = format!("{}:{}", host, port);

        TcpListener::bind(addr).await.unwrap()
    }

    /// Запуск приложения
    pub async fn run(self) {
        axum::serve(self.listener, self.router).await.unwrap()
    }
}
