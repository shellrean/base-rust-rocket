use crate::models::user::User;
use crate::repositories::user_repository;

pub fn get_user_detail(id: i32) -> Option<User> {
    user_repository::find_user_by_id(id)
}