use crate::{auth::*, Pet};
use aide::axum::IntoApiResponse;
use axum::extract::{Path, Json};
use axum::http::{Response, StatusCode, HeaderMap};
use crate::APP_STATE;
use serde::Deserialize;
use schemars::JsonSchema;


pub async fn route_get_pet(headers: HeaderMap, Path((user_uuid, pet_uuid)): Path<(String, String)>) -> impl IntoApiResponse  {
    // Verify token
    if !verify_token_header(&headers, &user_uuid).await {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized".to_string()) // Convert to String
            .unwrap();
    }

    let app_state = APP_STATE.lock().await;

    let pet = app_state.get_pet_by_uuid(&pet_uuid);

    if pet.is_none() {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Pet not found".to_string()) // Convert to String
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::OK)
            .body(serde_json::to_string(pet.unwrap()).unwrap()) // Convert to String
            .unwrap()
    }
}


#[derive(Deserialize, JsonSchema)]
pub struct PetUpdate {
    name: Option<String>,
    image: Option<u64>,
    species: Option<String>,
    pet_yard: Option<String>,
}

/// Handles updating the info about a pet, currently only name, image, species, and pet yard
/// The user must provide their UUID and token.
pub async fn route_update_pet(headers: HeaderMap, Path((user_uuid, pet_uuid)): Path<(String, String)>, payload: Json<PetUpdate>) -> impl IntoApiResponse  {
    // Verify token
    if !verify_token_header(&headers, &user_uuid).await {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized".to_string()) // Convert to String
            .unwrap();
    }

    let mut app_state = APP_STATE.lock().await;

    let pet = app_state.get_pet_by_uuid(&pet_uuid);

    if pet.is_none() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Pet not found".to_string()) // Convert to String
            .unwrap();
    }

    let mut pet = pet.unwrap().to_owned();

    if payload.name.is_some() {
        pet.set_name(payload.name.clone().unwrap());
    }

    if payload.image.is_some() {
        pet.set_image(payload.image.unwrap());
    }

    if payload.species.is_some() {
        pet.set_species(payload.species.clone().unwrap());
    }

    if payload.pet_yard.is_some() {
        pet.set_pet_yard(payload.pet_yard.clone().unwrap());
    }

    app_state.update_pet(pet.clone());

    Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&pet).unwrap()) // Convert to String
        .unwrap()
}

pub async fn route_delete_pet(headers: HeaderMap, Path((user_uuid, pet_uuid)): Path<(String, String)>) -> impl IntoApiResponse  {
    // Verify token
    if !verify_token_header(&headers, &user_uuid).await {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized".to_string()) // Convert to String
            .unwrap();
    }

    let mut app_state = APP_STATE.lock().await;

    let pet = app_state.get_pet_by_uuid(&pet_uuid);

    if pet.is_none() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Pet not found".to_string()) // Convert to String
            .unwrap();
    }

    let pet = pet.unwrap().to_owned();

    // Remove the pet from the user's pet list
    let mut user = app_state.get_user_by_uuid(&user_uuid).unwrap().clone();

    user.remove_pet(pet.get_uuid());

    app_state.update_user(user);

    app_state.delete_pet(&pet.get_uuid());

    Response::builder()
        .status(StatusCode::OK)
        .body("Pet deleted".to_string()) // Convert to String
        .unwrap()
}

pub async fn route_create_pet(headers: HeaderMap, user_uuid: Path<String>, payload: Json<PetUpdate>) -> impl IntoApiResponse  {
    // Verify token
    if !verify_token_header(&headers, &user_uuid).await {
        return Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("Unauthorized".to_string()) // Convert to String
            .unwrap();
    }

    let mut app_state = APP_STATE.lock().await;

    let pet = Pet::new(payload.name.clone().unwrap(), payload.species.clone().unwrap(), payload.image.unwrap(), payload.pet_yard.clone());

    app_state.update_pet(pet.clone());

    // Add the pet to the user's pet list
    let mut user = app_state.get_user_by_uuid(&user_uuid).unwrap().clone();

    user.add_pet(pet.get_uuid());

    app_state.update_user(user);

    Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_string(&pet).unwrap()) // Convert to String
        .unwrap()
}