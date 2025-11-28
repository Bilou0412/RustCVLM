use uuid::Uuid;
use serde::Serialize;

#[derive(Debug,Clone,Serialize)]
pub struct User
{
    id : Uuid,
    email : String,
    username : String
}

impl User{
    pub fn new(username:String,email:String)->Self{
        return Self {
            id:Uuid::new_v4(),email,username,
        }
    }

    pub fn id(&self) -> &Uuid{
        return &self.id
    }

    pub fn email(&self) -> &str{
        return &self.email
    }
    
    pub fn username(&self) ->&str{
        return &self.username
    }
}