use serde::Deserialize;
use crate::domain::user::User;

#[derive(Deserialize,Debug)]
pub struct CreateUserRequest{
    pub username: String,
    pub email: String
}

pub fn create_user(request:CreateUserRequest) -> User{
    return User::new(request.username, request.email)
}