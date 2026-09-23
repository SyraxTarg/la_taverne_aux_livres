use crate::AppState;
use axum::{Router, routing::get};
use std::sync::Arc;

use crate::api::controller::user_readings;
use crate::api::controller::users;

pub fn user_router() -> Router<Arc<AppState>> {
    Router::new()
    .route("/me", get(users::get_me))
    .route("/{id}", get(users::get_user_by_id))
    .route("/{id}/readings",get(user_readings::get_readings_by_user_id))
}
