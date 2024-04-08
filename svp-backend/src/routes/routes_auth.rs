use crate::{auth::*, User};
use aide::axum::IntoApiResponse;
use axum::extract::{Path, Json};
use axum::http::{Response, StatusCode, HeaderMap};
use crate::APP_STATE;
use serde::Deserialize;
use schemars::JsonSchema;


#[derive(Deserialize, JsonSchema)]
pub struct Login {
    username: String,
    password: String,
}

/// Handles the login of a user.
/// The user must provide their username and password.
pub async fn route_login(payload: Json<Login>) -> impl IntoApiResponse  {
    let username = payload.username.clone();
    let password = payload.password.clone();
    
    // Get json from request
    let app_state = APP_STATE.lock().await;

    let user = app_state.get_user_by_username(&username);

    if user.is_none() {
        tracing::warn!(
            "User not found: {}",
            username
        );
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("User / password combination not found".to_string()) // Convert to String
            .unwrap();
    }

    let user = user.unwrap().clone();

    if !user.compare_password(&password) {
        tracing::warn!(
            "Invalid password for user: {}",
            username
        );
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("User / password combination not found".to_string()) // Convert to String
            .unwrap();
    }

    drop(app_state);

    let mut app_state = APP_STATE.lock().await;

    let token = app_state.create_token(&user);

    let response_body = user.for_user_with_token(token);

    Response::builder()
        .status(StatusCode::OK)
        .body(response_body) // Convert to String
        .unwrap()}


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
pub async fn route_verify(Path((uuid, token)): Path<(String, String)>) -> impl IntoApiResponse  {
    verify(uuid.to_string(), token.to_string()).await
}

pub async fn route_refresh(Path((uuid, token)): Path<(String, String)>) -> impl IntoApiResponse  {
    refresh(uuid.to_string(), token.to_string()).await
}
