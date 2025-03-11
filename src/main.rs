extern crate tokio;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(|| async { "practicestax!" }));
    let localhost_url = "127.0.0.1:3000";

    let listener = tokio::net::TcpListener::bind(localhost_url)
        .await?;

    println!("Server running on {localhost_url}");
    axum::serve(listener, app)
        .await?;

    Ok(())
}
