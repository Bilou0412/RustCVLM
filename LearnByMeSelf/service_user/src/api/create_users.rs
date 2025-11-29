use crate::application::create_user;
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use axum::debug_handler;


#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct RegisterUserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

#[debug_handler]
pub async fn register_user_handler(
    Json(request): Json<RegisterUserRequest>,
) -> Json<RegisterUserResponse> {
    let user = create_user(request.email, request.username);
    Json(RegisterUserResponse {
        id: user.id().clone(),
        username: user.username().to_string(),
        email: user.email().to_string(),
    })
}
