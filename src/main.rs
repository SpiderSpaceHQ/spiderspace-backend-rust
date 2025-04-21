use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Create the router
    let app = Router::new().route("/", get(health_check));

    // Define the server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on {}", addr);

    // Start the server
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

// Health check handler
async fn health_check() -> &'static str {
    "Backend is running!"
}