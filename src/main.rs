use infrastructure::api::server;

#[tokio::main]
async fn main() {
    server::run().await.unwrap();
}
