use axum::{response::IntoResponse, Router};
use axum::routing::{get, post, delete};
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
mod utils;

use crate::auth::*;
use crate::encryption::*;
use crate::structs::*;
use crate::utils::*;

use crate::routes::routes_auth::*;
use crate::routes::routes_users::*;

// Create an arc mutex of AppState
pub static APP_STATE: Lazy<Arc<Mutex<AppState>>> =
    Lazy::new(|| Arc::new(Mutex::new(AppState::new())));

#[tokio::main]
async fn main() {
    // Load state.json into APP_STATE
    let state = std::fs::read_to_string("state.json");

    if let Ok(state) = state {
        *APP_STATE.lock().await = serde_json::from_str(&state).unwrap();
    }

    // Decide on what address to run the server
    let addr = if cfg!(debug_assertions) {
        SocketAddr::from(([127, 0, 0, 1], 3000))
    } else {
        SocketAddr::from(([0, 0, 0, 0], 3000))
    };

    println!("Listening on {}", addr);

    let app: Router = Router::new()
        .route("/", get(index))
        // Routes for authentication
        .route("/auth/login", post(route_login))
        .route("/auth/signup", post(route_signup))
        .route("/auth/logout/:token", post(route_logout))
        .route("/auth/refresh_token/:token", post(route_verify))

        // Routes for users. These routes require a token
        .route("/users/:uuid", get(route_get_user).delete(route_delete_user));
        /*
    
        // Routes for pets. These routes require a token
        .route("/pets/pet", get(route_get_pet))
        .route("/pets/pet", post(route_update_pet))
        .route("/pets/pet", delete(route_delete_pet))

        // Routes for pet yards. These routes require a token
        .route("/pet_yards/pet_yard", get(route_get_pet_yard))
        .route("/pet_yards/pet_yard", post(route_update_pet_yard))
        .route("/pet_yards/pet_yard", delete(route_delete_pet_yard))

        // Public routes, which return a 
        .route("/public/user", get(route_get_public_user))
        .route("/public/pet", get(route_get_public_pet))
        .route("/public/pet_yard", get(route_get_public_pet_yard));
         */
        



    // If the paths are not found, create the pem files
    if !std::path::Path::new("cert.pem").exists() {
        let (cert, key) = create_cert().unwrap();

        // Write the pem files
        std::fs::write("cert.pem", cert).unwrap();
        std::fs::write("key.pem", key).unwrap();
    }

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
