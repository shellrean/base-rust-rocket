use rocket::get;
use rocket::serde::json::Json;

use crate::models::user;
use crate::services::user_service;

#[get("/user/<id>")]
pub fn get_user(id: i32) -> Json<user::User> {
    let users = user_service::get_user_detail(id);

    Json(users.unwrap())
}

#[post("/user", format = "json", data = "<user>")]
pub fn store_user(user: Json<user::User>) {
    println!("id: {}", user.id);
    println!("username: {}", user.username);
    println!("email: {}", user.email);
}