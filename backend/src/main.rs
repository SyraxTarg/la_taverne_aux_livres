pub mod api;
pub mod proxy;

use axum::Router;
use reqwest::Client;
use std::env;
use std::sync::Arc;
use dotenv::dotenv;
use api::router::books::books_router;
use sea_orm::DatabaseConnection;

pub struct AppState {
    pub api_key: String,
    pub api_url: String,
    pub http_client: Client,
    pub db_pool: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    // 1. Chargement des variables d'environnement
    dotenv().ok();

    let api_key = env::var("GOOGLE_BOOKS_API_KEY").expect("Clé manquante dans le .env");
    let api_url = env::var("GOOGLE_BOOKS_API_URL").expect("URL manquante dans le .env");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL manquante dans le .env");

    // 2. Initialisation de la connexion à la base de données PostgreSQL
    println!("⏳ Connexion à PostgreSQL via SeaORM...");
    let db_pool = sea_orm::Database::connect(&database_url)
        .await
        .expect("❌ Impossible de se connecter à PostgreSQL.");
    println!("✅ Connecté avec SeaORM !");

    // 👇 Appel de l'initialisation de la table au démarrage
    crate::api::repo::categories::creer_table_si_inexistante(&db_pool).await;

    // 3. Création de l'état partagé (AppState)
    let state = Arc::new(AppState {
        api_key,
        api_url,
        http_client: Client::new(),
        db_pool, // 👈 On injecte le pool de connexion ici
    });

    // 4. Configuration du routeur Axum
    let app = Router::new()
        .nest("/api", books_router()) 
        .with_state(state);

    // 5. Démarrage du serveur
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Serveur démarré sur http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}