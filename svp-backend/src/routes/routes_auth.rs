use crate::auth::*;
use axum::extract::{Path, Json};
use serde::Deserialize;
use aide::axum::IntoApiResponse;
use schemars::JsonSchema;


#[derive(Deserialize, JsonSchema)]
pub struct Login {
    username: String,
    password: String,
}

/// Handles the login of a user.
/// The user must provide their username and password.
pub async fn route_login(payload: Json<Login>) -> impl IntoApiResponse  {
    // Get json from request
    login(payload.username.clone(), payload.password.clone()).await
}


#[derive(Deserialize, JsonSchema)]
pub struct Signup {
    username: String,
    email: String,
    password: String,
}

/// Handles the signup of a user.
/// The user must provide their username, email, and password.
pub async fn route_signup(payload: Json<Signup>) -> impl IntoApiResponse  {
    signup(payload.username.clone(), payload.email.clone(), payload.password.clone()).await
}

/// Handles the logout of a user.
/// The user must provide their UUID and token.
/// The server will remove the token from the list of valid tokens.
pub async fn route_logout(token: Path<String>) -> impl IntoApiResponse  {
    logout(token.to_string()).await
}

/// Handles the verification of a token.
/// The user must provide their token.
pub async fn route_verify(uuid: Path<String>, token: Path<String>) -> impl IntoApiResponse  {
    verify(uuid.to_string(), token.to_string()).await
}

pub async fn route_refresh(uuid: Path<String>, token: Path<String>) -> impl IntoApiResponse  {
    refresh(uuid.to_string(), token.to_string()).await
}
