use crate::models::user::NewUser;
use crate::routes::AppState;
use crate::services::user_services::{UserError, UserService};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserHandler {
    service: Arc<UserService>,
}

impl UserHandler {
    pub fn new(service: UserService) -> Self {
        UserHandler {
            service: Arc::new(service),
        }
    }
}

pub async fn get_users_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.user_handler.service.get_users() {
        Ok(users) => (StatusCode::OK, Json(json!({ "data": users }))).into_response(),
        Err(e) => match e {
            UserError::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": msg,
                    "status": 500
                })),
            )
                .into_response(),
            UserError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": msg,
                    "status": 404
                })),
            )
                .into_response(),
            UserError::ValidationError(_) => todo!(),
            UserError::HashError(_) => todo!(),
        },
    }
}

pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<NewUser>,
) -> impl IntoResponse {
    match state.user_handler.service.create_user(payload) {
        Ok(user) => (StatusCode::CREATED, Json(json!({ "data": user }))).into_response(),
        Err(e) => match e {
            UserError::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": msg,
                    "status": 500
                })),
            )
                .into_response(),
            UserError::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": msg,
                    "status": 400
                })),
            )
                .into_response(),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Unexpected error occurred",
                    "status": 500
                })),
            )
                .into_response(),
        },
    }
}

pub async fn get_user_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.user_handler.service.get_user(id) {
        Ok(user) => (StatusCode::OK, Json(json!({ "data": user }))).into_response(),
        Err(e) => match e {
            UserError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": msg,
                    "status": 404
                })),
            )
                .into_response(),
            UserError::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": msg,
                    "status": 500
                })),
            )
                .into_response(),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Unexpected error occurred",
                    "status": 500
                })),
            )
                .into_response(),
        },
    }
}

pub async fn update_user_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<NewUser>,
) -> impl IntoResponse {
    match state.user_handler.service.update_user(id, payload) {
        Ok(user) => (StatusCode::OK, Json(json!({ "data": user }))).into_response(),
        Err(e) => match e {
            UserError::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": msg,
                    "status": 500
                })),
            )
                .into_response(),
            UserError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": msg,
                    "status": 404
                })),
            )
                .into_response(),
            UserError::ValidationError(_) => todo!(),
            UserError::HashError(_) => todo!(),
        },
    }
}

pub async fn delete_user_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.user_handler.service.delete_user(id) {
        Ok(_) => (StatusCode::OK, Json(json!({ "data": "User deleted" }))).into_response(),
        Err(e) => match e {
            UserError::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": msg,
                    "status": 500
                })),
            )
                .into_response(),
            UserError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": msg,
                    "status": 404
                })),
            )
                .into_response(),
            UserError::ValidationError(_) => todo!(),
            UserError::HashError(_) => todo!(),
        },
    }
}
