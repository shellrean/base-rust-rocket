#[macro_use]
extern crate rocket;

mod models;
mod repositories;
mod routes;
mod services;

use routes::user;

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![user::get_user, user::store_user])
        .launch().await.expect("Failed to launch rocket");
}