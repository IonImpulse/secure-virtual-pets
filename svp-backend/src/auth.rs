/*

This file handling login and signup of users.

*/



use crate::structs::User;

use crate::APP_STATE;
use axum::http::{self, Response, StatusCode};
use aide::axum::IntoApiResponse;





pub async fn verify_token(token: &str, uuid: &str) -> bool {
    let app_state = APP_STATE.lock().await;

    if app_state.tokens.contains_key(token) {
        let user_token = app_state.tokens.get(token).unwrap();

        // Check if token is valid
        user_token.is_valid() && user_token.get_uuid() == uuid
    } else {
        false
    }
}

pub async fn verify_token_header(headers: &http::HeaderMap, uuid: &str) -> bool {
    let token = headers.get("X-Auth-Key");

    if token.is_none() {
        return false;
    }

    let token = token.unwrap().to_str().unwrap().to_string();

    verify_token(&token, uuid).await
}


pub async fn signup(username: String, email: String, password: String) -> impl IntoApiResponse  {
    let app_state = APP_STATE.lock().await;

    if app_state.get_user_by_username(&username).is_some() {
        return Response::builder()
            .status(StatusCode::CONFLICT)
            .body("Username already exists".to_string()) // Convert to String
            .unwrap();
    }

    let user = User::new(username.to_string(), email.to_string(), password.to_string());

    drop(app_state);

    let mut app_state = APP_STATE.lock().await;

    app_state.users.insert(user.get_uuid(), user);

    Response::builder()
        .status(StatusCode::OK)
        .body("User created".to_string()) // Convert to String
        .unwrap()
}

pub async fn logout(token: String) -> impl IntoApiResponse  {
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

pub async fn verify(token: String, uuid: String) -> impl IntoApiResponse  {
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

pub async fn refresh(token: String, _uuid: String) -> impl IntoApiResponse  {
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