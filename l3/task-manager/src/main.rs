use axum::{routing::{get, post}, Router};
use tokio::{net::TcpListener, sync::Mutex};
use redis::aio::MultiplexedConnection;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::sync::Arc;
use task_manager::{
    handlers::{complete_task, create_task, get_notifications, get_task},
    AppState,
};

async fn init_redis_client() -> MultiplexedConnection {
    let redis_url = std::env::var("REDIS_URL").unwrap();
    let client = redis::Client::open(redis_url).unwrap();
    client.get_multiplexed_async_connection().await.unwrap()
}

async fn init_db_pool() -> sqlx::PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap()
}

async fn init_router() -> Router {
    let db_pool = init_db_pool().await;
    let redis_client = Mutex::new(init_redis_client().await);
    let state = Arc::new(AppState { db_pool, redis_client });

    Router::new()
        .route("/tasks", post(create_task))
        .route("/tasks/:id", get(get_task))
        .route("/tasks/:id/complete", post(complete_task))
        .route("/notifications", get(get_notifications))
        .with_state(state)
}

async fn init_tcp_listener() -> TcpListener {
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    let addr = format!("{}:{}", host, port);

    TcpListener::bind(addr).await.unwrap()
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let router = init_router().await;
    let listener = init_tcp_listener().await;

    axum::serve(listener, router).await.unwrap()
}
