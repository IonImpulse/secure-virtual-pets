use crate::{auth::*, PetYard};
use aide::axum::IntoApiResponse;
use axum::extract::{Path, Json};
use axum::http::{Response, StatusCode, HeaderMap};
use crate::APP_STATE;
use serde::Deserialize;
use schemars::JsonSchema;

pub async fn route_get_pet_yard(headers: HeaderMap, Path((user_uuid, pet_yard_uuid)): Path<(String, String)>) -> impl IntoApiResponse  {
    // Verify token
    if !verify_token_header(&headers, &user_uuid).await {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized".to_string()) // Convert to String
            .unwrap();
    }

    let app_state = APP_STATE.lock().await;

    let pet_yard = app_state.get_pet_yard_by_uuid(&pet_yard_uuid);

    if pet_yard.is_none() {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Pet yard not found".to_string()) // Convert to String
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::OK)
            .body(serde_json::to_string(pet_yard.unwrap()).unwrap()) // Convert to String
            .unwrap()
    }
}

#[derive(Deserialize, JsonSchema)]
pub struct PetYardUpdate {
    name: Option<String>,
    image: Option<u64>,
}

pub async fn route_update_pet_yard(headers: HeaderMap, Path((user_uuid, pet_yard_uuid)): Path<(String, String)>, payload: Json<PetYardUpdate>) -> impl IntoApiResponse  {
    // Verify token
    if !verify_token_header(&headers, &user_uuid).await {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized".to_string()) // Convert to String
            .unwrap();
    }

    let mut app_state = APP_STATE.lock().await;

    let pet_yard = app_state.get_pet_yard_by_uuid(&pet_yard_uuid);

    if pet_yard.is_none() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Pet yard not found".to_string()) // Convert to String
            .unwrap();
    }

    let mut pet_yard = pet_yard.unwrap().to_owned();

    if payload.name.is_some() {
        pet_yard.set_name(payload.name.clone().unwrap());
    }

    if payload.image.is_some() {
        pet_yard.set_image(payload.image.unwrap());
    }

    app_state.update_pet_yard(pet_yard.clone());

    Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&pet_yard).unwrap()) // Convert to String
        .unwrap()
}

pub async fn route_delete_pet_yard(headers: HeaderMap, Path((user_uuid, pet_yard_uuid)): Path<(String, String)>) -> impl IntoApiResponse  {
    // Verify token
    if !verify_token_header(&headers, &user_uuid).await {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized".to_string()) // Convert to String
            .unwrap();
    }

    let mut app_state = APP_STATE.lock().await;

    let pet_yard = app_state.get_pet_yard_by_uuid(&pet_yard_uuid);

    if pet_yard.is_none() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Pet yard not found".to_string()) // Convert to String
            .unwrap();
    }

    let pet_yard = pet_yard.unwrap().to_owned();

    // Remove pet yard from user
    let mut user = app_state.get_user_by_uuid(&user_uuid).unwrap().to_owned();

    user.remove_owned_pet_yard(pet_yard.get_uuid());

    app_state.update_user(user);
    
    app_state.delete_pet_yard(&pet_yard.get_uuid());

    Response::builder()
        .status(StatusCode::OK)
        .body("Pet yard deleted".to_string()) // Convert to String
        .unwrap()
}

pub async fn route_create_pet_yard(headers: HeaderMap, user_uuid: Path<String>, payload: Json<PetYardUpdate>) -> impl IntoApiResponse  {
    // Verify token
    if !verify_token_header(&headers, &user_uuid).await {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized".to_string()) // Convert to String
            .unwrap();
    }

    let mut app_state = APP_STATE.lock().await;

    let pet_yard = PetYard::new(payload.name.clone().unwrap(), user_uuid.to_string(), payload.image.unwrap());

    app_state.update_pet_yard(pet_yard.clone());

    // Add pet yard to user
    let mut user = app_state.get_user_by_uuid(&user_uuid).unwrap().to_owned();

    user.add_owned_pet_yard(pet_yard.get_uuid());

    app_state.update_user(user);

    Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&pet_yard).unwrap()) // Convert to String
        .unwrap()
}

pub async fn route_add_member_to_pet_yard(headers: HeaderMap, Path((user_uuid, pet_yard_uuid, member_uuid)): Path<(String, String, String)>) -> impl IntoApiResponse {
    // Verify token
    if !verify_token_header(&headers, &user_uuid).await {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized".to_string())
            .unwrap();
    }

    let mut app_state = APP_STATE.lock().await;

    let pet_yard = app_state.get_pet_yard_by_uuid(&pet_yard_uuid);

    if pet_yard.is_none() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Pet yard not found".to_string()) // Convert to String
            .unwrap();
    }

    let mut pet_yard = pet_yard.unwrap().to_owned();

    pet_yard.add_member(member_uuid);

    app_state.update_pet_yard(pet_yard.clone());

    Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&pet_yard).unwrap()) // Convert to String
        .unwrap()
}

pub async fn route_remove_member_from_pet_yard(headers: HeaderMap, Path((user_uuid, pet_yard_uuid, member_uuid)): Path<(String, String, String)>) -> impl IntoApiResponse  {
    // Verify token
    if !verify_token_header(&headers, &user_uuid).await {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized".to_string()) // Convert to String
            .unwrap();
    }

    let mut app_state = APP_STATE.lock().await;

    let pet_yard = app_state.get_pet_yard_by_uuid(&pet_yard_uuid);

    if pet_yard.is_none() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Pet yard not found".to_string()) // Convert to String
            .unwrap();
    }

    let mut pet_yard = pet_yard.unwrap().to_owned();

    pet_yard.remove_member(member_uuid);

    app_state.update_pet_yard(pet_yard.clone());

    Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&pet_yard).unwrap()) // Convert to String
        .unwrap()
}

pub async fn route_add_pet_to_pet_yard(headers: HeaderMap, (user_uuid, pet_yard_uuid, pet_uuid): (Path<String>, Path<String>, Path<String>)) -> impl IntoApiResponse  {
    // Verify token
    if !verify_token_header(&headers, &user_uuid).await {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized".to_string()) // Convert to String
            .unwrap();
    }

    let mut app_state = APP_STATE.lock().await;

    let pet_yard = app_state.get_pet_yard_by_uuid(&pet_yard_uuid);

    if pet_yard.is_none() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Pet yard not found".to_string()) // Convert to String
            .unwrap();
    }

    let mut pet_yard = pet_yard.unwrap().to_owned();

    pet_yard.add_pet(pet_uuid.to_string());

    app_state.update_pet_yard(pet_yard.clone());

    println!("{:?}", pet_yard);

    Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&pet_yard).unwrap()) // Convert to String
        .unwrap()
}

pub async fn route_remove_pet_from_pet_yard(headers: HeaderMap, Path((user_uuid, pet_yard_uuid, pet_uuid)): Path<(String, String, String)>) -> impl IntoApiResponse  {
    // Verify token
    if !verify_token_header(&headers, &user_uuid).await {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized".to_string()) // Convert to String
            .unwrap();
    }

    let mut app_state = APP_STATE.lock().await;

    let pet_yard = app_state.get_pet_yard_by_uuid(&pet_yard_uuid);

    if pet_yard.is_none() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Pet yard not found".to_string()) // Convert to String
            .unwrap();
    }

    let mut pet_yard = pet_yard.unwrap().to_owned();

    pet_yard.remove_pet(pet_uuid.to_string());

    app_state.update_pet_yard(pet_yard.clone());

    Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&pet_yard).unwrap()) // Convert to String
        .unwrap()
}