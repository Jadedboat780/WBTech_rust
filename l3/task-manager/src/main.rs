use task_manager::{init_router, init_tcp_listener};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let router = init_router().await;
    let listener = init_tcp_listener().await;

    axum::serve(listener, router).await.unwrap()
}
