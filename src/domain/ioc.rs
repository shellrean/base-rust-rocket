use crate::domain::user::UserService;

pub struct ServiceContainer {
    pub user_service: Box<dyn UserService>
}