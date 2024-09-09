use axum::{routing, Router};
use tower_http::cors::{Any, CorsLayer};
use tokio::net::TcpListener;
use tokio_postgres::{Client, NoTls};

use std::sync::Arc;

use l0::{
    endpoints::{hello_word, handler_404, get_order, create_order},
    AppState
};


/// Инициализация подключения к базе данных
async fn init_db_connect() -> Client {
    let db_url = std::env::var("DATABASE_URL").expect("Error getting connection");
    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(error) = connection.await {
            eprintln!("Connection error: {}", error);
        }
    });

    client
}

/// Инициализация CORS
async fn init_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any)
}

/// Инициализация главного роутера
async fn init_router() -> Router {
    let client = init_db_connect().await;
    let state = Arc::new(AppState { client });
    let cors = init_cors().await;

    Router::new()
        .route("/", routing::get(hello_word))
        .route("/orders/:id", routing::get(get_order))
        .route("/orders", routing::post(create_order))
        .fallback(handler_404)
        .with_state(state)
        .layer(cors)
}

async fn init_tcp_listener() -> TcpListener {
    let host = std::env::var("HOST").expect("Host don`t set");
    let port = std::env::var("PORT").expect("Port don`t set");
    let addr = format!("{}:{}", host, port);

    TcpListener::bind(addr).await.expect("the address is busy")
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let router = init_router().await;
    let listener = init_tcp_listener().await;

    axum::serve(listener, router).await.unwrap()
}
