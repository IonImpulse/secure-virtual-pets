use axum::{response::IntoResponse, routing::get, Router};
use axum_server::tls_openssl;

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

    // Create TLS configuration with native_tls
    let tls = tls::TlsAcceptor::from(
        tokio_rustls::TlsAcceptor::from(
            tokio_rustls::rustls::ServerConfig::new(tokio_rustls::rustls::NoClientAuth::new()),
        )
        .configure(|c| c.set_single_cert(
            tokio_rustls::rustls::internal::pemfile::certs(&mut std::io::BufReader::new(
                std::fs::File::open("cert.pem").unwrap(),
            ))
            .unwrap(),
            tokio_rustls::rustls::internal::pemfile::rsa_private_keys(&mut std::io::BufReader::new(
                std::fs::File::open("key.pem").unwrap(),
            ))
            .unwrap()
            .remove(0),
        ))
        .expect("invalid key/cert files"),
    );
}

async fn index() -> impl IntoResponse {
    "Hello, World!"
}
