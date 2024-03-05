use axum::{response::IntoResponse, routing::get, Router};
use axum_server::tls_rustls;

use axum_server::tls_rustls::RustlsConfig;
use tower_http::{
    compression::CompressionLayer, limit::RequestBodyLimitLayer, trace::TraceLayer,
    validate_request::ValidateRequestHeaderLayer,
};

use once_cell::sync::Lazy;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

mod auth;
mod encryption;
mod routes;
mod structs;

use crate::auth::*;
use crate::encryption::*;
use crate::routes::*;
use crate::structs::*;

// Create an arc mutex of AppState
pub static APP_STATE: Lazy<Arc<Mutex<AppState>>> =
    Lazy::new(|| Arc::new(Mutex::new(AppState::new())));

#[tokio::main]
async fn main() {
    // Load state.json into APP_STATE
    let state = std::fs::read_to_string("state.json").unwrap();
    let state: AppState = serde_json::from_str(&state).unwrap();
    *APP_STATE.lock().await = state;

    // Decide on what address to run the server
    let addr = if cfg!(debug_assertions) {
        SocketAddr::from(([127, 0, 0, 1], 3000))
    } else {
        SocketAddr::from(([0, 0, 0, 0], 3000))
    };

    println!("Listening on {}", addr);

    let app: Router = Router::new().route("/", get(index));

    let config: RustlsConfig = RustlsConfig::from_pem_file(
        "cert.pem",
        "key.pem",
    ).await.unwrap();

    // create app with bind_tls
    axum_server::bind_rustls(
        addr,
        config,
    ).serve(app.into_make_service())
    .await
    .unwrap();
}

async fn index() -> impl IntoResponse {
    "Hello, World!"
}
