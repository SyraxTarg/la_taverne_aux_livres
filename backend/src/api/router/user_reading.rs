use crate::AppState;
use axum::{
    Router,
    routing::{delete, put},
};
use std::sync::Arc;

use crate::api::controller::user_readings;

pub fn user_reading_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", put(user_readings::create_reading))
        .route("/{book_id}", delete(user_readings::delete_reading))
}
