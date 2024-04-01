use aide::redoc::Redoc;
use axum::{http, Extension, Json};
use axum_server::tls_rustls::RustlsConfig;

use aide::{
    axum::{
        routing::{delete, get, patch, post},
        ApiRouter, IntoApiResponse,
    },
    openapi::{OpenApi, Tag},
    transform::TransformOpenApi,
};

use once_cell::sync::Lazy;
use std::{net::SocketAddr, time::Duration};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::trace::{self, TraceLayer};
use tracing::{Level, Span};

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
use crate::routes::routes_pet_yards::*;
use crate::routes::routes_pets::*;
use crate::routes::routes_public::*;
use crate::routes::routes_users::*;

// Create an arc mutex of AppState
pub static APP_STATE: Lazy<Arc<Mutex<AppState>>> =
    Lazy::new(|| Arc::new(Mutex::new(AppState::new())));

#[tokio::main]
async fn main() {
    aide::gen::on_error(|error| {
        println!("{error}");
    });

    aide::gen::extract_schemas(true);

    let mut api = OpenApi::default();

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

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(CustomMakeSpan)
        .on_response(CustomOnResponse);

    let app = ApiRouter::new()
        .route("/", get(index))
        .route("/redoc", Redoc::new("/api.json").axum_route())
        // Routes for authentication
        .api_route("/auth/login", post(route_login))
        .api_route("/auth/signup", post(route_signup))
        .api_route("/auth/logout/:uuid/:token", post(route_logout))
        .api_route("/auth/refresh_token/:uuid/:token", post(route_refresh))
        // Routes for users.
        .api_route(
            "/users/:uuid",
            get(route_get_user)
                .patch(route_update_user)
                .delete(route_delete_user),
        )
        // Routes for pets.
        .api_route(
            "/users/:user_uuid/pets/:pet_uuid",
            get(route_get_pet)
                .patch(route_update_pet)
                .delete(route_delete_pet),
        )
        .api_route("/users/:user_uuid/pets/new", post(route_create_pet))
        // Routes for petyards
        .api_route(
            "/users/:user_uuid/pet_yards/:pet_yard_uuid",
            get(route_get_pet_yard)
                .patch(route_update_pet_yard)
                .delete(route_delete_pet_yard),
        )
        .api_route(
            "/users/:user_uuid/pet_yards/new",
            post(route_create_pet_yard),
        )
        .api_route(
            "/users/:user_uuid/pet_yards/:pet_yard_uuid/member/:member_uuid",
            patch(route_add_member_to_pet_yard).delete(route_remove_member_from_pet_yard),
        )
        .api_route(
            "/users/:user_uuid/pet_yards/:pet_yard_uuid/pet/:pet_uuid",
            patch(route_add_pet_to_pet_yard).delete(route_remove_pet_from_pet_yard),
        )
        // Public routes for users
        .api_route("/public/user/:uuid", get(route_get_public_user))
        .api_route("/public/pet/:uuid", get(route_get_public_pet))
        .api_route("/public/pet_yard/:uuid", get(route_get_public_pet_yard))
        .route("/api.json", get(route_api_json))
        .layer(
            trace_layer
        );

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
        .serve(
            app.finish_api_with(&mut api, api_docs)
                .layer(Extension(api))
                .into_make_service(),
        )
        .await
        .unwrap();
}

#[derive(Debug, Clone)]
struct CustomMakeSpan;

impl<B> trace::MakeSpan<B> for CustomMakeSpan {
    fn make_span(&mut self, request: &http::Request<B>) -> Span {
        let remote_addr = request
            .extensions()
            .get::<SocketAddr>()
            .map(SocketAddr::to_string)
            .unwrap_or_else(|| "-".to_string());

        tracing::info_span!(
            "request",
            http.method = %request.method(),
            http.uri = %request.uri(),
            http.remote_addr = %remote_addr,
            otel.kind = "server",
        )
    }
}

#[derive(Debug, Clone)]
struct CustomOnResponse;

impl<B> trace::OnResponse<B> for CustomOnResponse {
    fn on_response(self, response: &http::Response<B>, latency: Duration, span: &Span) {
        let status = response.status();
        let length = response
            .headers()
            .get(http::header::CONTENT_LENGTH)
            .and_then(|val| val.to_str().ok())
            .unwrap_or("-");

        span.record("http.status_code", &tracing::field::display(status.as_u16()));
        span.record("http.response_content_length", &length);

        let latency_secs = latency.as_secs_f32();

        tracing::info!(
            status = %status.as_u16(),
            length = %length,
            latency_secs = format!("{:.3}s", latency_secs),
            "request processed",
        );
    }
}

async fn route_api_json(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}
fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Secure Virtual Pet Backend API")
        .summary("A secure virtual pet backend API.")
        .description("")
        .security_scheme(
            "User Token",
            aide::openapi::SecurityScheme::ApiKey {
                location: aide::openapi::ApiKeyLocation::Header,
                name: "X-Auth-Key".into(),
                description: Some("User session token. Verified with the UUID of the user".into()),
                extensions: Default::default(),
            },
        )
}

async fn index() -> impl IntoApiResponse {
    "Hello, World!"
}
