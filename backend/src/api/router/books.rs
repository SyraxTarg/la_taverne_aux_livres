use axum::{routing::get, Router};
use std::sync::Arc;
use crate::AppState; 

use crate::api::controller::books::{get_books_by_recherche, get_book_by_id, get_books_by_category_id};

pub fn books_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_books_by_recherche))
        .route("/{id}", get(get_book_by_id))
        // 👇 NOUVELLE ROUTE : Recherche par ID de catégorie
        .route("/category/{id}", get(get_books_by_category_id)) 
}