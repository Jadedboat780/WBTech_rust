use tokio_postgres::Client;

pub mod api_response;
pub mod endpoints;
pub mod models;
pub mod queries;

pub struct AppState {
    pub client: Client,
}
