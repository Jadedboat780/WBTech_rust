use axum::{routing, Router};
use tokio::net::TcpListener;
use std::sync::Arc;
use multi_user_chat::{
    handlers::{join_room, leave_room, send_message, get_messages},
    AppState,
};


#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::new());

    let router = Router::new()
        .route("/join", routing::post(join_room))
        .route("/leave/:room_id/:user_id", routing::post(leave_room))
        .route("/send", routing::post(send_message))
        .route("/messages/:room_id", routing::get(get_messages))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap()
}
