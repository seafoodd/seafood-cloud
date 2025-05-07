use axum::{
    extract::{Multipart, Path as PathExtractor},
    http::{header, Response, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router
    ,
};
use serde::{Deserialize, Serialize};
use tokio::fs as tokio_fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Welcome to Seafood Cloud!" }))
        .route("/download/{file_id}", get(download_file))
        .route("/upload", post(upload_file));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Deserialize)]
struct FileMeta {
    original_name: String,
}

async fn download_file(PathExtractor(file_id): PathExtractor<String>) -> impl IntoResponse {
    let filepath = format!("./uploads/{}", file_id);
    let meta_path = format!("./uploads/{}.json", file_id);

    let meta_json = match tokio_fs::read_to_string(&meta_path).await {
        Ok(content) => content,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    let meta: FileMeta = match serde_json::from_str(&meta_json) {
        Ok(meta) => meta,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let file = match tokio_fs::read(&filepath).await {
        Ok(file_content) => file_content,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", meta.original_name),
        )
        .body(file.into())
        .unwrap()
}

async fn upload_file(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field.file_name().unwrap_or("unknown").to_string();
        let uuid = Uuid::new_v4().to_string();

        let filepath = format!("./uploads/{}", uuid);
        let meta_path = format!("{}.json", filepath);

        let mut file = File::create(&filepath).await.unwrap();
        let data = field.bytes().await.unwrap();
        file.write_all(&data).await.unwrap();

        let meta = FileMeta { original_name: filename.clone() };
        let mut meta_file = File::create(&meta_path).await.unwrap();
        let meta_json = serde_json::to_string(&meta).unwrap();
        meta_file.write_all(meta_json.as_bytes()).await.unwrap();

        println!("File uploaded: {} as ID {}", filename, uuid);
    }
}