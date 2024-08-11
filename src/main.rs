use axum::{
    extract::Request,
    Json,
    routing::post,
    routing::get,
    Router,
};
use axum::response::{IntoResponse, Response};
use rustc_version_runtime::version;

use serde::Serialize;


use tower::util::ServiceExt;
use tower_http::{
    services::{ServeFile},
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route_service("/", get(|request: Request| async {
            let service = ServeFile::new("static/index.html");
            let result = service.oneshot(request).await;
            result
        }).post(|| async { "Hello, World!!" }))
        .route_service("/favicon.ico", ServeFile::new("static/favicon.ico"))
        .route_service("/favicon.svg", ServeFile::new("static/favicon.svg"))
        .route_service("/robots.txt", ServeFile::new("static/robots.txt"))
        .route_service("/status.json", get(get_status))
        ;

    // run our app with hyper, listening globally on port 3000

    // get address from environment variable
    let address = std::env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string());

    // get port from environment variable
    let port = std::env::var("PORT").unwrap_or_else(|_| "4000".to_string());

    let listen = format!("{}:{}", address, port);

    let listener = tokio::net::TcpListener::bind(listen).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct StatusInfo {
    success: bool,
    message: String,
    tech: String,
    timestamp: String,
    lastmod: String,
    commit: String,
}

async fn get_status() -> Response {

    let tech = format!("Rust {}", version());
    let timestamp = chrono::Utc::now().to_rfc3339();
    let lastmod = std::env::var("LASTMOD").unwrap_or_else(|_| "(local)".to_string());
    let commit = std::env::var("COMMIT").unwrap_or_else(|_| "(local)".to_string());

    let status = StatusInfo {
        success: true,
        message: "OK".to_string(),
        tech: tech.to_string(),
        timestamp: timestamp.to_string(),
        lastmod: lastmod.to_string(),
        commit: commit.to_string(),
    };
    Json(status).into_response()
}
