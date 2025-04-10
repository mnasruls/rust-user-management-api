use crate::config::database::establish_connection;
use crate::handlers::role_handler::RoleHandler;
use crate::handlers::user_handler::UserHandler;
use crate::repositories::user_repository::UserRepository;
use crate::routes::create_router;
use crate::services::role_services::RoleService;
use crate::services::user_services::UserService;
use axum::Router;
use repositories::role_repository;
use tokio::net::TcpListener;

mod config;
mod handlers;
mod models;
mod repositories;
mod routes;
mod schema;
mod services;

#[tokio::main]
async fn main() {
    let pool = establish_connection();

    let user_repository = UserRepository::new(pool.clone());
    let role_repository = role_repository::RoleRepository::new(pool.clone());
    let role_service = RoleService::new(role_repository.clone());
    let user_service = UserService::new(user_repository, role_repository.clone());
    let user_handler = UserHandler::new(user_service);
    let role_handler = RoleHandler::new(role_service);

    let app: Router = create_router(user_handler, role_handler);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
