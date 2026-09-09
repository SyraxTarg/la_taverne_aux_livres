use axum::{routing::post, Router};
use std::sync::Arc;
use crate::AppState;

use crate::api::controller::comments::{create_comment};

pub fn comments_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_comment))
}