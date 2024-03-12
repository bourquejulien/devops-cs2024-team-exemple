use axum::{Router, routing::get,};
use tokio::signal;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

mod controllers;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let app = Router::new();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await.unwrap();
}

