use axum::{
    Router,
    routing::{get, post},
};
use crate::handlers::file_handlers::{download_file, upload_file};

pub mod handlers;
pub mod models;
pub mod services;
pub mod utils;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Welcome to Seafood Cloud!" }))
        .route("/download/{file_id}", get(download_file))
        .route("/upload", post(upload_file));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
