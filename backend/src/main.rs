// backend/src/main.rs
//
// Axum server that serves the compiled Yew frontend as static files
// and provides a simple API for future dynamic features.

use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::path::PathBuf;
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

#[derive(Serialize, Clone)]
struct Book {
    title: String,
    author: String,
    description: String,
    cover_url: String,
    month: String,
    goodreads_url: String,
    partiful_url: Option<String>,
    is_current: bool,
}

async fn get_books() -> Json<Vec<Book>> {
    let books = vec![Book {
        title: "Headshot".to_string(),
        author: "Rita Bullwinkel".to_string(),
        description: "An electrifying novel about a women's boxing tournament in Reno, Nevada, featuring eight teenage competitors.".to_string(),
        cover_url: "https://m.media-amazon.com/images/S/compressed.photo.goodreads.com/books/1691077222i/174156218.jpg".to_string(),
        month: "April 2026".to_string(),
        goodreads_url: "https://www.goodreads.com/book/show/174156218".to_string(),
        partiful_url: Some("https://partiful.com/e/Z0UI3U0H7Ooe1XgYdfHr?c=AfUZfvML".to_string()),
        is_current: true,
    }];

    Json(books)
}

async fn health() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    let api = Router::new()
        .route("/api/books", get(get_books))
        .route("/api/health", get(health));

    // Resolve dist/ relative to the Cargo manifest directory (project root).
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let dist_dir: PathBuf = [manifest_dir, "..", "dist"].iter().collect();
    let index_html: PathBuf = dist_dir.join("index.html");

    let spa = ServeDir::new(&dist_dir).fallback(ServeFile::new(&index_html));

    let app = api
        .fallback_service(spa)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🦀 Element Book Club server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
