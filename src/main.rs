use axum::{
    extract::Request,
    routing::post,
    routing::get,
    Router,
};

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