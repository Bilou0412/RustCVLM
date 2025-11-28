use axum::Json;
use crate::application::create_user::{CreateUserRequest,create_user};
use crate::domain::user::User;

pub async fn  create_user_handler(Json(payload):Json<CreateUserRequest>) -> Json<User>
{
    return Json(create_user(payload))
}