use crate::domain::User;

pub fn create_user(email: String, username: String) -> User {
    User::new(username, email)
}
