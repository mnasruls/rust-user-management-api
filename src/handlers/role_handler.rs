use crate::models::role::NewRole;
use crate::routes::AppState;
use crate::services::role_services::{RoleError, RoleService};
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
pub struct RoleHandler {
    service: Arc<RoleService>,
}

impl RoleHandler {
    pub fn new(service: RoleService) -> Self {
        RoleHandler {
            service: Arc::new(service),
        }
    }
}

pub async fn get_roles_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.role_handler.service.get_roles() {
        Ok(roles) => (StatusCode::OK, Json(json!({ "data": roles }))).into_response(),
        Err(e) => match e {
            RoleError::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": msg,
                    "status": 500
                })),
            )
                .into_response(),
            RoleError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": msg,
                    "status": 404
                })),
            )
                .into_response(),
        },
    }
}

pub async fn create_role_handler(
    State(state): State<AppState>,
    Json(payload): Json<NewRole>,
) -> impl IntoResponse {
    match state.role_handler.service.create_role(payload) {
        Ok(role) => (StatusCode::CREATED, Json(json!({ "data": role }))).into_response(),
        Err(e) => match e {
            RoleError::DatabaseError(msg) => (
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

pub async fn get_role_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.role_handler.service.get_role(id) {
        Ok(role) => (StatusCode::OK, Json(json!({ "data": role }))).into_response(),
        Err(e) => match e {
            RoleError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": msg,
                    "status": 404
                })),
            )
                .into_response(),
            RoleError::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": msg,
                    "status": 500
                })),
            )
                .into_response(),
        },
    }
}

pub async fn update_role_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<NewRole>,
) -> impl IntoResponse {
    match state.role_handler.service.update_role(id, payload) {
        Ok(role) => (StatusCode::OK, Json(json!({ "data": role }))).into_response(),
        Err(e) => match e {
            RoleError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": msg,
                    "status": 404
                })),
            )
                .into_response(),
            RoleError::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": msg,
                    "status": 500
                })),
            )
                .into_response(),
        },
    }
}

pub async fn delete_role_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.role_handler.service.delete_role(id) {
        Ok(role) => (StatusCode::OK, Json(json!({ "data": role }))).into_response(),
        Err(e) => match e {
            RoleError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": msg,
                    "status": 404
                })),
            )
                .into_response(),
            RoleError::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": msg,
                    "status": 500
                })),
            )
                .into_response(),
        },
    }
}
