use axum::{extract::Request, middleware::Next, response::Response};
use tracing::info;

/// Логирование запросов
pub async fn log_requests(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();

    let response = next.run(req).await;
    info!("Response: {} {}", method, uri);

    response
}
