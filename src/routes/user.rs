use rocket::get;
use rocket::serde::json::Json;
use rocket::State;
use crate::domain::user::{User};
use crate::domain::ioc::ServiceContainer;

#[get("/user/<id>")]
pub fn get_user(service: &State<ServiceContainer>, id: i32) -> Json<User> {
    let users = service.user_service.show(id);
    Json(users.unwrap())
}

#[post("/user", format = "json", data = "<user>")]
pub fn store_user(user: Json<User>) {
    println!("id: {}", user.id);
    println!("username: {}", user.username);
    println!("email: {}", user.email);
}