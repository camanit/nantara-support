mod ai;
mod api;
mod diagnostics;
mod license;
mod utils;

use axum::{
    body::Body,
    http::{header, HeaderValue, StatusCode, Uri},
    response::{IntoResponse, Response},
    Router,
};
use rust_embed::RustEmbed;
use std::fs;
use std::io;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[derive(RustEmbed)]
#[folder = "web/"]
struct Assets;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Auto-load .env file if present
    if let Ok(content) = fs::read_to_string(".env") {
        for line in content.lines() {
            if let Some((key, val)) = line.split_once('=') {
                std::env::set_var(key.trim(), val.trim());
            }
        }
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .nest("/", api::routes::create_router())
        .fallback(static_handler)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));
    let has_ai_key = !std::env::var("GEMINI_API_KEY").unwrap_or_default().is_empty();
    let lic_status = license::verifier::load_local_license();

    println!("============================================================");
    println!("🚀 Nantara Support Agent & Control Panel (Rust)");
    println!("------------------------------------------------------------");
    println!("🌐 Dashboard Control Panel : http://localhost:3030");
    println!("🔌 REST API Base Endpoint  : http://localhost:3030/api/status");
    println!("🤖 AI Helpdesk Endpoint    : http://localhost:3030/api/ai/chat");
    println!("🔑 Gemini AI Key Status    : {}", if has_ai_key { "ACTIVE (Connected)" } else { "OFFLINE (Local Engine)" });
    println!("🛡️ License Tier            : {} [{}]", lic_status.tier_name, lic_status.client_name);
    println!("============================================================");

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(err) => {
            println!("❌ Gagal membuka port 3030: {}", err);
            println!("⚠️ Nantara Support mungkin sudah berjalan di background.");
            println!("\nBuka http://localhost:3030 di browser Anda.");
            println!("Tekan ENTER untuk keluar...");
            let mut pause = String::new();
            let _ = io::stdin().read_line(&mut pause);
            return;
        }
    };

    // Otomatis membuka browser default ke http://localhost:3030 saat .exe di-double click
    tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", "http://localhost:3030"])
            .spawn();
    });

    if let Err(e) = axum::serve(listener, app).await {
        println!("Server Error: {}", e);
        let mut pause = String::new();
        let _ = io::stdin().read_line(&mut pause);
    }
}

/// Handler static file yang di-embed langsung ke dalam single .exe binary
async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();
    if path.is_empty() {
        path = "index.html".to_string();
    }

    match Assets::get(&path) {
        Some(content) => {
            let mime = mime_guess::from_path(&path).first_or_octet_stream();
            Response::builder()
                .header(header::CONTENT_TYPE, HeaderValue::from_str(mime.as_ref()).unwrap())
                .body(Body::from(content.data))
                .unwrap()
        }
        None => match Assets::get("index.html") {
            Some(content) => Response::builder()
                .header(header::CONTENT_TYPE, HeaderValue::from_static("text/html"))
                .body(Body::from(content.data))
                .unwrap(),
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("404 Not Found"))
                .unwrap(),
        },
    }
}
