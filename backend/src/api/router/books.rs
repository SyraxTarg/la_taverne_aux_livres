use axum::{routing::get, Router};
use std::sync::Arc;
use crate::AppState;

// On importe tes deux fonctions du contrôleur
use crate::api::controller::books::{get_books_by_recherche, get_book_by_id};

pub fn books_router() -> Router<Arc<AppState>> {
    Router::new()
        // La route de recherche (ex: /api/books?recherche=dune)
        .route("/books", get(get_books_by_recherche))

        // La NOUVELLE route pour un ID précis (ex: /api/books/V9TODwAAQBAJ)
        // Les deux points ":" indiquent à Axum que c'est une variable dynamique
        .route("/books/{id}", get(get_book_by_id))
}