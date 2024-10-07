use tokio::sync::Mutex;

pub mod handlers;
pub mod models;
pub mod notifications;

pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub redis_client: Mutex<redis::aio::MultiplexedConnection>,
}
