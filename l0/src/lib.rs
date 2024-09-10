use dashmap::DashMap;
use std::sync::Arc;
use tokio::time::interval;
use tokio::time::Duration;
use crate::models::GetOrder;

pub mod api_response;
pub mod endpoints;
pub mod models;
pub mod queries;

pub struct AppState {
    pub client: tokio_postgres::Client,
    pub cache: DashMap<String, GetOrder>,
}


/// Очистка кеша каждые n минут
pub async fn cache_cleanup_task(state: Arc<AppState>, n: u64) {
    let mut interval = interval(Duration::from_secs(n));

    loop {
        interval.tick().await;

        log::info!("Очистка кэша...");
        state.cache.clear();
    }
}
