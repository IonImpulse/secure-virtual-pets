/*

This file handling login and signup of users.

*/

use std::f32::consts::E;

use crate::structs::User;
use crate::encryption::{encrypt, decrypt};
use crate::APP_STATE;
use axum::http::{self, Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use axum::body::HttpBody;

/// Handles the login of a user.
/// The user must provide their username and password.
/// The password is hashed and salted.
/// The server will return a token if the login is successful.
pub async fn login(username: String, password: String) -> impl IntoResponse {
    let app_state = APP_STATE.lock().await;

    let user = app_state.get_user_by_username(&username);

    if user.is_none() {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("User not found".to_string()) // Convert to String
            .unwrap();
    }

    let user = user.unwrap();

    if !user.compare_password(&password) {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Incorrect password".to_string()) // Convert to String
            .unwrap();
    }

    let mut app_state = APP_STATE.lock().await;

    let token = app_state.create_token(&user);

    Response::builder()
        .status(StatusCode::OK)
        .body(token.to_string()) // Convert to String
        .unwrap()
}


pub async fn signup(username: &str, email: &str, password: &str) -> impl IntoResponse {
    let app_state = APP_STATE.lock().await;

    if app_state.get_user_by_username(username).is_some() {
        return Response::builder()
            .status(StatusCode::CONFLICT)
            .body("Username already exists".to_string()) // Convert to String
            .unwrap();
    }

    let user = User::new(username.to_string(), email.to_string(), password.to_string());

    let mut app_state = APP_STATE.lock().await;

    app_state.users.insert(user.get_uuid(), user);

    Response::builder()
        .status(StatusCode::OK)
        .body("User created".to_string()) // Convert to String
        .unwrap()
}