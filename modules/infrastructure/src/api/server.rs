use anyhow::Result;
use axum::{routing::get, Router};

pub async fn run() -> Result<()> {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    axum::Server::bind(&"0.0.0.0:1234".parse().unwrap()).serve(app.into_make_service()).await.unwrap();

    Ok(())
}
