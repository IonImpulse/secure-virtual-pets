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

pub async fn verify_token(token: &str, uuid: &str) -> bool {
    let app_state = APP_STATE.lock().await;

    if app_state.tokens.contains_key(token) {
        let user_token = app_state.tokens.get(token).unwrap();

        // Check if token is valid
        return user_token.is_valid() && user_token.get_uuid() == uuid;
    } else {
        return false;
    }
}

pub async fn verify_token_header(headers: &http::HeaderMap, uuid: &str) -> bool {
    let token = headers.get("authorization");

    if token.is_none() {
        return false;
    }

    let token = token.unwrap().to_str().unwrap().to_string();

    verify_token(&token, uuid).await
}

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


pub async fn signup(username: String, email: String, password: String) -> impl IntoResponse {
    let app_state = APP_STATE.lock().await;

    if app_state.get_user_by_username(&username).is_some() {
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

pub async fn logout(token: String) -> impl IntoResponse {
    let mut app_state = APP_STATE.lock().await;

    if app_state.tokens.contains_key(&token) {
        app_state.tokens.remove(&token);
    } else {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Invalid token".to_string()) // Convert to String
            .unwrap();
    }

    Response::builder()
        .status(StatusCode::OK)
        .body("Logged out".to_string()) // Convert to String
        .unwrap()
}

pub async fn verify(token: String, uuid: String) -> impl IntoResponse {
    let valid = verify_token(&token, &uuid).await;

    if valid {
        Response::builder()
            .status(StatusCode::OK)
            .body("Token is valid".to_string()) // Convert to String
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Invalid token".to_string()) // Convert to String
            .unwrap()
    }
}

pub async fn refresh(token: String, uuid: String) -> impl IntoResponse {
    let mut app_state = APP_STATE.lock().await;

    if app_state.tokens.contains_key(&token) {
        if let Some(user_token) = app_state.tokens.get_mut(&token) {
            user_token.refresh();

            return Response::builder()
                .status(StatusCode::OK)
                .body("Token refreshed".to_string()) // Convert to String
                .unwrap()
        }
    }

    Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Invalid token".to_string()) // Convert to String
            .unwrap()
}