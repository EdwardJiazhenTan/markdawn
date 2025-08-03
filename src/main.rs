mod data;
mod events;
mod parser;
mod renderer;
mod watcher;
mod websocket;

use axum::{
    Json, Router,
    response::Html,
    routing::{get, post},
};
use websocket::ConnectionManager;
use watcher::FileWatcher;
use data::{Block, Document, Element};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct MarkdownRequest {
    content: String,
}

#[derive(Serialize)]
struct MarkdownResponse {
    html: String,
    success: bool,
    message: String,
}

#[tokio::main]
async fn main() {
    // Initialize the connection manager for WebSocket handling
    let connection_manager = ConnectionManager::new();

    // Initialize file watcher
    let mut file_watcher = FileWatcher::new(connection_manager.clone());
    
    // Start watching for file changes in a background task
    tokio::spawn(async move {
        // Watch current directory for .md files
        if let Err(e) = file_watcher.start_watching(".").await {
            eprintln!("Failed to start file watcher: {}", e);
        }
    });

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/demo", get(serve_demo))
        .route("/api/convert", post(convert_markdown))
        // WebSocket endpoint for real-time updates
        .route("/ws", get(websocket::websocket_handler))
        // Pass connection_manager as application state
        .with_state(connection_manager.clone())
        .nest_service("/static", ServeDir::new("static"));

    let listener = match tokio::net::TcpListener::bind("127.0.0.1:5000").await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to port 5000: {}", e);
            eprintln!("Maybe another instance is running? Try: pkill -f markdawn");
            return;
        }
    };

    println!("Server Running on http://localhost:5000");
    println!("Open your browser and edit test.md or README.md to see real-time updates!");

    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Server error: {}", e);
    }
}

async fn serve_index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

async fn serve_demo() -> Html<String> {
    let sample_doc = create_sample_document();
    Html(sample_doc.to_html())
}

fn create_sample_document() -> Document {
    use data::*;
    Document {
        blocks: vec![
            Block::Title {
                level: 1,
                content: vec![Element::PlainText("Sample Document".to_string())],
            },
            Block::Paragraph(vec![
                Element::PlainText("This is a ".to_string()),
                Element::Bold("bold".to_string()),
                Element::PlainText(" and ".to_string()),
                Element::Italic("italic".to_string()),
                Element::PlainText(" text example.".to_string()),
            ]),
        ],
    }
}

async fn convert_markdown(Json(payload): Json<MarkdownRequest>) -> Json<MarkdownResponse> {
    match parser::parse_markdown(&payload.content) {
        Ok(document) => {
            let html = document.to_html();
            Json(MarkdownResponse {
                html,
                success: true,
                message: "Conversion successful".to_string(),
            })
        }
        Err(e) => Json(MarkdownResponse {
            html: String::new(),
            success: false,
            message: format!("Parse error: {}", e),
        }),
    }
}