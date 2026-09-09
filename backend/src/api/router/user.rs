use axum::{routing::get, Router};
use std::sync::Arc;
use crate::AppState;

use crate::api::controller::users;

pub fn user_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/me", get(users::get_me))
}