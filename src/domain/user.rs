use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

pub trait UserRepository: Send + Sync {
    fn find_by_id(&self, id: i32) -> Option<User>;
}

pub trait UserService: Send + Sync {
    fn show(&self, id: i32) -> Option<User>;
}