use axum::{routing::get, Router};
use std::sync::Arc;
use crate::AppState;
use crate::api::controller::recommandations::get_recommendations;

pub fn recommandations_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_recommendations))
}

