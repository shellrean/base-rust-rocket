use crate::models::user::User;

pub fn find_user_by_id(id: i32) -> Option<User> {
    Some(User {
        id,
        username: format!("shellrean-{}", id),
        email: format!("shellrean-{}@shellrean.id", id),
    })
}