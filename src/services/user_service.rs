use crate::domain::user::{User, UserService, UserRepository};

pub struct UserServiceImpl {
    pub user_repo: Box<dyn UserRepository>,
}

impl UserService for UserServiceImpl {
    fn show(&self, id: i32) -> Option<User> {
        self.user_repo.find_by_id(id)
    }
}