use axum::{routing::post, Router};
use std::sync::Arc;
use crate::AppState;

use crate::api::controller::auth::{register, login};

pub fn auth_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}