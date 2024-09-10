use axum::{routing, Router};
use tokio::net::TcpListener;
use tokio_postgres::{Client, NoTls};
use tower_http::cors::{Any, CorsLayer};

use std::sync::Arc;
use dashmap::DashMap;
use l0::{cache_cleanup_task, endpoints::{create_order, get_order, handler_404, hello_word}, AppState};

/// Инициализация подключения к базе данных
async fn init_db_connect() -> Client {
    let db_url = std::env::var("DATABASE_URL").expect("Ошибка при получении соединения");
    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(error) = connection.await {
            log::error!("Ошибка подключения: {}", error);
            std::process::abort()
        }
    });

    log::info!("Успешное подключение к базе данных");
    client
}

/// Инициализация CORS
async fn init_cors() -> CorsLayer {
    log::info!("Настройка CORS");
    CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any)
}

/// Инициализация общего состояния приложения
async fn init_state() -> AppState {
    let client = init_db_connect().await;
    let cache = DashMap::new();

    let state = AppState { client, cache };

    state
}

/// Инициализация главного роутера
async fn init_router() -> Router {
    let state = Arc::new(init_state().await);
    let cors = init_cors().await;

    // tokio::spawn(cache_cleanup_task(state.clone(), 600));

    log::info!("Настройка маршрутов");
    Router::new()
        .route("/", routing::get(hello_word))
        .route("/orders/:track_number", routing::get(get_order))
        .route("/orders", routing::post(create_order))
        .fallback(handler_404)
        .with_state(state)
        .layer(cors)
}

/// Инициализация TCP слушателя
async fn init_tcp_listener() -> TcpListener {
    let host = std::env::var("HOST").expect("Хост не установлен");
    let port = std::env::var("PORT").expect("Порт не установлен");
    let addr = format!("{}:{}", host, port);

    log::info!("Запуск TCP слушателя на адресе: {}", addr);
    TcpListener::bind(addr).await.expect("Адрес занят")
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let router = init_router().await;
    let listener = init_tcp_listener().await;

    log::info!(
        "Сервер запущен на http://{}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, router).await.unwrap()
}
