use axum::{routing, Router};
use task11::endpoints::{
    create_event, delete_event, events_for_day, events_for_month, events_for_week, handler_404,
    update_event,
};
use tokio::net::TcpListener;

/// Инициализация главного роутера
async fn init_router() -> Router {
    Router::new()
        .route("/create_event ", routing::post(create_event))
        .route("/update_event ", routing::post(update_event))
        .route("/delete_event ", routing::post(delete_event))
        .route("/events_for_day", routing::get(events_for_day))
        .route("/week", routing::get(events_for_week))
        .route("/month", routing::get(events_for_month))
        .fallback(handler_404)
}

/// Инициализация TCP слушателя
async fn init_tcp_listener() -> TcpListener {
    let host = std::env::var("HOST").expect("Хост не установлен");
    let port = std::env::var("PORT").expect("Порт не установлен");
    let addr = format!("{}:{}", host, port);

    TcpListener::bind(addr).await.expect("Адрес занят")
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let router = init_router().await;
    let listener = init_tcp_listener().await;

    axum::serve(listener, router).await.unwrap()
}
