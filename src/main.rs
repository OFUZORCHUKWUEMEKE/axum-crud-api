use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use route::create_router;
use tower_http::cors::CorsLayer;
mod handler;
mod models;
mod response;
mod route;
async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Build Simple CRUD API in Rust using axum";

    let json_response = serde_json::json!({
        "status":"success",
        "message":MESSAGE
    });
    let _ = Json(json_response);
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    let app = create_router().layer(cors);
    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
