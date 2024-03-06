use axum::response::IntoResponse;
use axum::extract::Path;
use axum::http::{Response, StatusCode};
use crate::APP_STATE;


pub async fn route_get_public_user(uuid: Path<String>) -> impl IntoResponse {
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
            .body(user.unwrap().for_public()) // Convert to String
            .unwrap();
    }
}

pub async fn route_get_public_pet(pet_uuid: Path<String>) -> impl IntoResponse {
    let app_state = APP_STATE.lock().await;

    let pet = app_state.get_pet_by_uuid(&pet_uuid);

    if pet.is_none() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Pet not found".to_string()) // Convert to String
            .unwrap();
    } else {
        return Response::builder()
            .status(StatusCode::OK)
            .body(pet.unwrap().for_public()) // Convert to String
            .unwrap();
    }
}

pub async fn route_get_public_pet_yard(pet_yard_uuid: Path<String>) -> impl IntoResponse {
    let app_state = APP_STATE.lock().await;

    let pet_yard = app_state.get_pet_yard_by_uuid(&pet_yard_uuid);

    if pet_yard.is_none() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Pet yard not found".to_string()) // Convert to String
            .unwrap();
    } else {
        return Response::builder()
            .status(StatusCode::OK)
            .body(pet_yard.unwrap().for_public()) // Convert to String
            .unwrap();
    }
}