use crate::auth::*;
use aide::axum::IntoApiResponse;
use axum::extract::{Path, Json};
use axum::http::{Response, StatusCode, HeaderMap};
use crate::APP_STATE;
use serde::Deserialize;
use schemars::JsonSchema;

/// Handles getting the info about a user
pub async fn route_get_user(headers: HeaderMap, uuid: Path<String>) -> impl IntoApiResponse  {
    // Verify token
    if !verify_token_header(&headers, &uuid).await {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized".to_string()) // Convert to String
            .unwrap();
    }

    let app_state = APP_STATE.lock().await;

    let user = app_state.get_user_by_uuid(&uuid);

    if user.is_none() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("User not found".to_string()) // Convert to String
            .unwrap();
    } else {
        return Response::builder()
            .status(StatusCode::OK)
            .body(user.unwrap().for_user()) // Convert to String
            .unwrap();
    }
}

#[derive(Deserialize, JsonSchema)]
pub struct UserUpdate {
    email: Option<String>,
    password: Option<String>,

}

/// Handles updating the info about a user, currently only email and password
pub async fn route_update_user(headers: HeaderMap, uuid: Path<String>, payload: Json<UserUpdate>) -> impl IntoApiResponse  {
    // Verify token
    if !verify_token_header(&headers, &uuid).await {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized".to_string()) // Convert to String
            .unwrap();
    }

    let mut app_state = APP_STATE.lock().await;

    let user = app_state.get_user_by_uuid(&uuid);

    if user.is_none() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("User not found".to_string()) // Convert to String
            .unwrap();
    }

    let mut user = user.unwrap().clone();

    if let Some(email) = &payload.email {
        user.set_email(email.clone());
    }

    if let Some(password) = &payload.password {
        user.set_password(password.clone());
    }

    app_state.update_user(user);

    Response::builder()
        .status(StatusCode::OK)
        .body("User updated".to_string()) // Convert to String
        .unwrap()
}

/// Handles deleting a user
pub async fn route_delete_user(headers: HeaderMap, uuid: Path<String>) -> impl IntoApiResponse  {
    // Verify token
    if !verify_token_header(&headers, &uuid).await {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized".to_string()) // Convert to String
            .unwrap();
    }

    let mut app_state = APP_STATE.lock().await;

    let user = app_state.get_user_by_uuid(&uuid);

    if user.is_none() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("User not found".to_string()) // Convert to String
            .unwrap();
    }

    let user = user.unwrap().to_owned(); // Drop the immutable borrow

    app_state.delete_user(user);

    Response::builder()
        .status(StatusCode::OK)
        .body("User deleted".to_string()) // Convert to String
        .unwrap()
}