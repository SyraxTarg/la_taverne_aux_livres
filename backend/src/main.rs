// On déclare nos dossiers comme modules Rust
pub mod api;
pub mod proxy;

use axum::{routing::get, Router};
use reqwest::Client;
use std::env;
use std::sync::Arc;
use dotenv::dotenv;

// On rend la structure publique (pub) pour que api/books.rs puisse l'utiliser
pub struct AppState {
    pub api_key: String,
    pub api_url: String,
    pub http_client: Client,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("GOOGLE_BOOKS_API_KEY").expect("Clé manquante");
    let api_url = env::var("GOOGLE_BOOKS_API_URL").expect("URL manquante");

    let state = Arc::new(AppState {
        api_key,
        api_url,
        http_client: Client::new(),
    });

    // On utilise la fonction get_books de notre module api
    let app = Router::new()
        .route("/api/books", get(api::books::get_books))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Serveur démarré sur http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}