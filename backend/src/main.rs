pub mod api;
pub mod proxy;

use api::dto::requests::auth::{LoginDto, RegisterDto};
use api::dto::requests::comment::{CreateCommentDto, UpdateCommentDto};
use api::dto::requests::user_reading::UserReadingDto;
use api::dto::responses::auth::LoginResponseDto;
use api::dto::responses::author::AuthorDto;
use api::dto::responses::book::BookDto;
use api::dto::responses::book_rating::BookRatingsStatsDto;
use api::dto::responses::categories::CategoryDto;
use api::dto::responses::comment::{
    CommentResponseDto, CommentWithRepliesDto, UserCommentResponseDto,
};
use api::dto::responses::recommendation::RecommendedBookDto;
use api::dto::responses::search_books::{BookSearchResponse, PaginationDto};
use api::dto::responses::user::UserResponseDto;
use api::dto::responses::user_reading::UserReadingResponseDto;
use api::router::auth::auth_router;
use api::router::books::books_router;
use api::router::comments::comments_router;
use api::router::recommandations::recommandations_router;
use api::router::user::user_router;
use api::router::user_reading::user_reading_router;
use axum::Router;
use dotenv::dotenv;
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use reqwest::Client;
use sea_orm::DatabaseConnection;
use std::env;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::controller::books::get_books_by_recherche,
        crate::api::controller::books::get_book_by_id,
        crate::api::controller::books::get_books_by_category_id,
        crate::api::controller::books::get_comments_by_book,
        crate::api::controller::books::get_book_ratings,
        crate::api::controller::auth::login,
        crate::api::controller::auth::register,
        crate::api::controller::users::get_me,
        crate::api::controller::comments::create_comment,
        crate::api::controller::comments::get_comment_by_id,
        crate::api::controller::comments::update_comment,
        crate::api::controller::comments::delete_comment,
        crate::api::controller::user_readings::create_reading,
        crate::api::controller::user_readings::get_readings_by_user_id,
        crate::api::controller::user_readings::delete_reading,
        crate::api::controller::recommandations::get_recommendations,
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
            UpdateCommentDto,
            CommentWithRepliesDto,
            UserReadingDto,
            UserReadingResponseDto,
            UserCommentResponseDto,
            BookRatingsStatsDto,
            RecommendedBookDto
        )
    ),
    tags(
        (name = "books", description = "Gestion du catalogue et recherche de livres"),
        (name = "auth", description = "Gestion de l'authentification"),
        (name = "utilisateurs", description = "Gestion des utilisateurs"),
        (name = "comments", description = "Gestion des commentaires"),
        (name = "lectures", description = "Gestion des lectures"),
        (name = "recommandations", description = "Système de recommandation basé sur le Machine Learning"),
    )
)]
struct ApiDoc;

pub struct AppState {
    pub api_key: String,
    pub api_url: String,
    pub http_client: Client,
    pub db_pool: DatabaseConnection,
    pub embedding_model: Arc<TextEmbedding>,
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
    crate::api::repo::roles::creer_table_et_roles_base(&db_pool)
        .await
        .expect("Erreur lors de l'initialisation des rôles");
    crate::api::repo::users::creer_table_si_inexistante(&db_pool).await;
    crate::api::repo::comments::creer_table_si_inexistante(&db_pool).await;
    crate::api::repo::user_reading::creer_table_si_inexistante(&db_pool).await;

    // 3. Initialisation du modèle de Machine Learning
    println!("⏳ Initialisation du modèle de Machine Learning (FastEmbed)...");
    let embedding_model = Arc::new(
        TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::MultilingualE5Small).with_show_download_progress(true),
        )
        .expect("❌ Impossible d'initialiser le modèle d'embedding."),
    );
    println!("✅ Modèle de Machine Learning prêt !");

    // 4. Création de l'état partagé (AppState)
    let state = Arc::new(AppState {
        api_key,
        api_url,
        http_client: Client::new(),
        db_pool,
        embedding_model,
    });

    // 5. Configuration du routeur Axum
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api/books", books_router())
        .nest("/api/auth", auth_router())
        .nest("/api/users", user_router())
        .nest("/api/comments", comments_router())
        .nest("/api/readings", user_reading_router())
        .nest("/api/recommandations", recommandations_router())
        .with_state(state);

    // 6. Démarrage du serveur
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap();
    let addr = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("🚀 Serveur démarré sur http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
