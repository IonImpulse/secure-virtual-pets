use axum::routing::{patch, get, post};
use axum::{response::IntoResponse, Router};
use axum_server::tls_rustls::RustlsConfig;

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
use crate::routes::routes_pets::*;
use crate::routes::routes_users::*;
use crate::routes::routes_pet_yards::*;
use crate::routes::routes_public::*;

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
        .route("/auth/logout/:uuid/:token", post(route_logout))
        .route("/auth/refresh_token/:uuid/:token", post(route_refresh))
        // Routes for users.
        .route(
            "/users/:uuid",
            get(route_get_user)
                .patch(route_update_user)
                .delete(route_delete_user),
        )
        // Routes for pets.
        .route(
            "/users/:user_uuid/pets/:pet_uuid",
            get(route_get_pet)
                .patch(route_update_pet)
                .delete(route_delete_pet)
        )
        .route(
            "/users/:user_uuid/pets/new",
            post(route_create_pet)
        )

        // Routes for petyards
        .route(
            "/users/:user_uuid/pet_yards/:pet_yard_uuid",
            get(route_get_pet_yard)
                .patch(route_update_pet_yard)
                .delete(route_delete_pet_yard)
        )
        .route(
            "/users/:user_uuid/pet_yards/new",
            post(route_create_pet_yard)
        )
        .route(
            "/users/:user_uuid/pet_yards/:pet_yard_uuid/member/:member_uuid",
            patch(route_add_member_to_pet_yard)
            .delete(route_remove_member_from_pet_yard)
        )
        .route(
            "/users/:user_uuid/pet_yards/:pet_yard_uuid/pet/:pet_uuid",
            patch(route_add_pet_to_pet_yard)
            .delete(route_remove_pet_from_pet_yard)
        )

        // Public routes for users
        .route("/public/user/:uuid", get(route_get_public_user))
        .route("/public/pet/:uuid", get(route_get_public_pet))
        .route("/public/pet_yard/:uuid", get(route_get_public_pet_yard));

    // If the paths are not found, create the pem files
    if !std::path::Path::new("cert.pem").exists() {
        let (cert, key) = create_cert().unwrap();

        // Write the pem files
        std::fs::write("cert.pem", cert).unwrap();
        std::fs::write("key.pem", key).unwrap();
    }

    let config: RustlsConfig = RustlsConfig::from_pem_file("cert.pem", "key.pem")
        .await
        .unwrap();

    // create app with bind_tls
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> impl IntoResponse {
    "Hello, World!"
}
