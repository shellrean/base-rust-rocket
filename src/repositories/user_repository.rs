use crate::domain::user::{User, UserRepository};

pub struct UserRepositoryImpl;

impl UserRepository for UserRepositoryImpl {
    fn find_by_id(&self, id: i32) -> Option<User> {
        Some(User {
            id,
            username: format!("shellrean-{}", id),
            email: format!("shellrean-{}@shellrean.id", id),
        })
    }
}
