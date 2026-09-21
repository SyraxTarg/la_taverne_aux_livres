use crate::AppState;
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

use crate::api::controller::comments::{
    create_comment, delete_comment, get_comment_by_id, update_comment,
};

pub fn comments_router() -> Router<Arc<AppState>> {
    Router::new().route("/", post(create_comment)).route(
        "/{id}",
        get(get_comment_by_id)
            .put(update_comment)
            .delete(delete_comment),
    )
}
