mod repositories;
mod routes;
mod services;
mod domain;

use rocket::{routes};

use routes::user;
use repositories::user_repository::UserRepositoryImpl;
use services::user_service::UserServiceImpl;

#[rocket::main]
async fn main() {
    let user_repo = UserRepositoryImpl;
    let user_service = UserServiceImpl{
        user_repo: Box::new(user_repo)
    };

    let service_container = domain::ioc::ServiceContainer{
        user_service: Box::new(user_service)
    };
    rocket::build()
        .manage(service_container)
        .mount("/", routes![user::get_user, user::store_user])
        .launch().await.expect("Failed to launch rocket");
}