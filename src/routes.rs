use crate::handlers::role_handler::{
    RoleHandler, create_role_handler, delete_role_handler, get_role_handler, get_roles_handler,
    update_role_handler,
};
use crate::handlers::user_handler::{
    UserHandler, create_user_handler, delete_user_handler, get_user_handler, get_users_handler,
    update_user_handler,
};
use axum::Json;
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use serde_json::json;

#[derive(Clone)]
pub struct AppState {
    pub user_handler: UserHandler,
    pub role_handler: RoleHandler,
}

pub fn create_router(user_handler: UserHandler, role_handler: RoleHandler) -> Router {
    let state = AppState {
        user_handler,
        role_handler,
    };

    Router::new()
        .route(
            "/health",
            get(|| async {
                Json(json!({
                    "message": "Server is up and running",
                    "status": 200
                }))
            }),
        )
        // User routes
        .route("/users", get(get_users_handler))
        .route("/users", post(create_user_handler))
        .route("/users/:id", get(get_user_handler))
        .route("/users/:id", put(update_user_handler))
        .route("/users/:id", delete(delete_user_handler))
        // Role routes
        .route("/roles", get(get_roles_handler))
        .route("/roles", post(create_role_handler))
        .route("/roles/:id", get(get_role_handler))
        .route("/roles/:id", put(update_role_handler))
        .route("/roles/:id", delete(delete_role_handler))
        .with_state(state)
}
