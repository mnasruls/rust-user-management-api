use super::{role_handler::RoleHandler, user_handler::UserHandler};

pub struct AppState {
    pub role_handler: RoleHandler,
    pub user_handler: UserHandler,
}
