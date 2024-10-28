use mini_social_network::App;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = App::new().await;
    app.run().await;
}
