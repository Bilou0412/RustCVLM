use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    id: Uuid,
    username: String,
    email: String,
}

impl User {
    pub fn new(username: String, email: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            email,
        }
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
}
