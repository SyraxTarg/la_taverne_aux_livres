pub mod api;
pub mod proxy;

use axum::Router;
use reqwest::Client;
use std::env;
use std::sync::Arc;
use dotenv::dotenv;
use api::router::books::books_router;
use api::router::auth::auth_router;
use api::router::user::user_router;
use api::router::comments::comments_router;
use sea_orm::DatabaseConnection;
use api::dto::responses::author::AuthorDto;
use api::dto::responses::book::BookDto;
use api::dto::responses::categories::CategoryDto;
use api::dto::requests::auth::{LoginDto, RegisterDto};
use api::dto::responses::search_books::{BookSearchResponse, PaginationDto};
use api::dto::responses::auth::{LoginResponseDto};
use api::dto::responses::user::UserResponseDto;
use api::dto::responses::comment::{CommentResponseDto, CommentWithRepliesDto};
use api::dto::requests::comment::CreateCommentDto;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::controller::books::get_books_by_recherche,
        crate::api::controller::books::get_book_by_id,
        crate::api::controller::books::get_books_by_category_id,
        crate::api::controller::auth::login,
        crate::api::controller::auth::register,
        crate::api::controller::users::get_me,
        crate::api::controller::comments::create_comment
    ),
    components(
        schemas(
            BookDto,
            CategoryDto,
            AuthorDto,
            PaginationDto,
            BookSearchResponse,
            LoginDto,
            RegisterDto,
            LoginResponseDto,
            UserResponseDto,
            CommentResponseDto,
            CreateCommentDto,
            CommentWithRepliesDto
        )
    ),
    tags(
        (name = "books", description = "Gestion du catalogue et recherche de livres"),
        (name = "auth", description = "Gestion de l'authentification"),
        (name = "utilisateurs", description = "Gestion des utilisateurs"),
        (name = "Commentaires", description = "Gestion des commentaires"),
    )
)]
struct ApiDoc;

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
    crate::api::repo::roles::creer_table_et_roles_base(&db_pool).await.expect("Erreur lors de l'initialisation des rôles");
    crate::api::repo::users::creer_table_si_inexistante(&db_pool).await;
    crate::api::repo::comments::creer_table_si_inexistante(&db_pool).await;

    // 3. Création de l'état partagé (AppState)
    let state = Arc::new(AppState {
        api_key,
        api_url,
        http_client: Client::new(),
        db_pool, // 👈 On injecte le pool de connexion ici
    });

    // 4. Configuration du routeur Axum
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api/books", books_router())
        .nest("/api/auth", auth_router())
        .nest("/api/users", user_router())
        .nest("/api/comments", comments_router())
        .with_state(state);

    // 5. Démarrage du serveur
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Serveur démarré sur http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}