use crate::auth::*;
use axum::response::IntoResponse;
use axum::extract::{Path, Json};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

/// Handles the login of a user.
/// The user must provide their username and password.
pub async fn route_login(payload: Json<Login>) -> impl IntoResponse {
    // Get json from request
    login(payload.username.clone(), payload.password.clone()).await
}


#[derive(Deserialize)]
pub struct Signup {
    username: String,
    email: String,
    password: String,
}

/// Handles the signup of a user.
/// The user must provide their username, email, and password.
pub async fn route_signup(payload: Json<Signup>) -> impl IntoResponse {
    signup(payload.username.clone(), payload.email.clone(), payload.password.clone()).await
}

/// Handles the logout of a user.
/// The user must provide their UUID and token.
/// The server will remove the token from the list of valid tokens.
pub async fn route_logout(token: Path<String>) -> impl IntoResponse {
    logout(token.to_string()).await
}

/// Handles the verification of a token.
/// The user must provide their token.
pub async fn route_verify(token: Path<String>) -> impl IntoResponse {
    verify(token.to_string()).await
}

pub async fn route_refresh(token: Path<String>) -> impl IntoResponse {
    refresh(token.to_string()).await
}
