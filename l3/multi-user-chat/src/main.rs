use axum::{routing, Router};
use multi_user_chat::{
    handlers::{create_room, create_user, get_messages, join_room, leave_room, send_message},
    AppState,
};
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::default());

    let router = Router::new()
        .route("/create_user/:room_name", routing::get(create_user))
        .route("/create_room/:room_name", routing::get(create_room))
        .route("/join", routing::post(join_room))
        .route("/leave/:room_id/:user_id", routing::post(leave_room))
        .route("/send", routing::post(send_message))
        .route("/messages/:room_id", routing::get(get_messages))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap()
}
