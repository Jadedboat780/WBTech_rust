use mini_social_network::{init_router, init_tcp_listener};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let listener = init_tcp_listener().await;
    let router = init_router().await;

    axum::serve(listener, router).await.unwrap();
}
