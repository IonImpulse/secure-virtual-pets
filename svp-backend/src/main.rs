use axum::{
    response::IntoResponse,
    routing::get,
    Router,
};

use tower_http::{
    compression::CompressionLayer, limit::RequestBodyLimitLayer, trace::TraceLayer,
    validate_request::ValidateRequestHeaderLayer,
};

use std::net::SocketAddr;

mod routes;
mod auth;
mod structs;
use crate::structs::*;

#[tokio::main]
async fn main() {
    // Decide on what address to run the server
    let addr = if cfg!(debug_assertions) {
        SocketAddr::from(([127, 0, 0, 1], 3000))
    } else {
        SocketAddr::from(([0, 0, 0, 0], 3000))
    };

    println!("Listening on {}", addr);

    let app: Router = Router::new().route("/", get(index));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> impl IntoResponse {
    "Hello, World!"
}
