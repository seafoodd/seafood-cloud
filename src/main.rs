use axum::{
    Router,
    extract::multipart::Multipart,
    routing::{get, post},
};
use tokio::fs::{File, create_dir_all};
use tokio::io::AsyncWriteExt;
use std::path::Path;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Welcome to Seafood Cloud!" }))
        .route(
            "/download/{capture}",
            get(|| async { "Welcome to Seafood Cloud!" }),
        )
        .route("/upload", post(upload_file));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn download_file() {}

async fn upload_file(mut multipart: Multipart) {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let filename = field.file_name().unwrap_or("unknown");
        let filepath = format!("./uploads/{}", filename);

        let dir_path = Path::new("./uploads");
        if !dir_path.exists() {
            create_dir_all(dir_path).await.unwrap();
        }

        let mut file = File::create(Path::new(&filepath)).await.unwrap();

        let data = field.bytes().await.unwrap();
        file.write_all(&data).await.unwrap();

        println!("File uploaded to: {}", filepath);
    }
}