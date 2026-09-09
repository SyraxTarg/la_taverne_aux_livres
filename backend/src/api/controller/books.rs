use axum::{
    extract::{State, Query, Path},
    Json
};
use serde::{Deserialize}; // 👈 Ajout de Serialize
use utoipa::{ToSchema, IntoParams};
use serde_json::{json, Number, Value};
use std::sync::Arc;
use std::collections::HashSet; // 👈 1. Import du HashSet pour les catégories

use crate::AppState;
use crate::proxy::search_books;
use crate::proxy::get_by_id;
use crate::api::dto::responses::book::BookDto;
use crate::api::dto::responses::search_books::{BookSearchResponse, PaginationDto};
use crate::api::dto::responses::author::AuthorDto;
use crate::api::dto::responses::categories::CategoryDto;
use crate::api::service::categories as categories_service;

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct ParametresRecherche {
    pub recherche: Option<String>,
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}


#[utoipa::path(
    get,
    path = "/api/books",
    params(ParametresRecherche),
    responses(
        (status = 200, description = "Liste des livres trouvés", body = BookSearchResponse),
        (status = 500, description = "Erreur de l'API Google Books")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "books"
)]
pub async fn get_books_by_recherche(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ParametresRecherche>,
) -> Result<Json<BookSearchResponse>, Json<Value>> {

    let limit = params.limit.unwrap_or(10).min(40);
    let offset = params.offset.unwrap_or(0); // 👈 Remis à 0 pour Google Books

    // 👇 2. On définit un mot-clé par défaut si l'URL est juste "/api/books"
    let mot_cle = params.recherche.unwrap_or_else(|| "livre".to_string());

    let resultat = search_books::search_books(
        &state.http_client,
        &state.api_url,
        &state.api_key,
        &mot_cle, // 👈 On utilise la variable sécurisée ici
        offset,
        limit
    ).await;

    match resultat {
        Ok(data) => {
            let total_items = data.get("totalItems")
                .and_then(|t| t.as_u64())
                .map(|t| t as usize)
                .unwrap_or(0);

            let mut livres = Vec::new();

            // 👇 3. Création du "panier" pour récolter les catégories sans doublons
            let mut categories_recoltees: HashSet<String> = HashSet::new();

            if let Some(items) = data.get("items").and_then(|i| i.as_array()) {
                for item in items {
                    let volume_info = &item["volumeInfo"];

                    let id = item["id"].as_str().unwrap_or("").to_string();
                    let title = volume_info["title"].as_str().unwrap_or("Titre inconnu").to_string();
                    let description = volume_info["description"].as_str().map(|s| s.to_string());
                    let maturity_rating = volume_info["maturityRating"].as_str().unwrap_or("").to_string();
                    let published_date = volume_info["publishedDate"].as_str().unwrap_or("").to_string();

                    let image_url = volume_info.get("imageLinks")
                        .and_then(|links| links.get("thumbnail"))
                        .and_then(|t| t.as_str())
                        .map(|s| s.to_string());

                    let page_count = volume_info["pageCount"]
                        .as_u64()
                        .map(Number::from)
                        .unwrap_or_else(|| Number::from(0));

                    let average_rating = volume_info["averageRating"]
                        .as_f64()
                        .and_then(Number::from_f64)
                        .unwrap_or_else(|| Number::from(0));

                    let ratings_count = volume_info["ratingsCount"]
                        .as_u64()
                        .map(Number::from)
                        .unwrap_or_else(|| Number::from(0));

                    let authors = volume_info["authors"]
                        .as_array()
                        .map(|auteurs_array| {
                            auteurs_array
                                .iter()
                                .filter_map(|auteur| auteur.as_str())
                                .map(|nom| AuthorDto { name: nom.to_string() })
                                .collect()
                        })
                        .unwrap_or_else(Vec::new);

                    // 👇 4. Modification de la gestion des catégories pour remplir le HashSet
                    let categories = volume_info["categories"]
                        .as_array()
                        .map(|categories_array| {
                            categories_array
                                .iter()
                                .filter_map(|cat| cat.as_str())
                                .flat_map(|cat_str| cat_str.split(" / "))
                                .map(|nom| {
                                    let nom_propre = nom.trim().to_string();

                                    // On insère la catégorie dans notre panier !
                                    categories_recoltees.insert(nom_propre.clone());

                                    CategoryDto { name: nom_propre }
                                })
                                .collect()
                        })
                        .unwrap_or_else(Vec::new);

                    livres.push(BookDto {
                        id,
                        title,
                        authors,
                        description,
                        image_url,
                        page_count,
                        categories,
                        average_rating,
                        ratings_count,
                        maturity_rating,
                        published_date,
                    });
                }
            }

            // 👇 5. Envoi des catégories récoltées vers la base de données
            // Attention : il faudra que ce module et cette fonction existent pour compiler !
            categories_service::sauvegarder_nouvelles_categories(
                &state.db_pool,
                categories_recoltees
            ).await;

            Ok(Json(BookSearchResponse {
                pagination: PaginationDto {
                    total_items,
                    offset,
                    limit,
                },
                livres,
            }))
        },
        Err(_) => {
            Err(Json(json!({ "erreur": "Impossible de contacter l'API Google Books" })))
        }
    }
}



#[utoipa::path(
    get,
    path = "/api/books/{id}",
    params(
        ("id" = String, Path, description = "L'identifiant unique du livre chez Google Books")
    ),
    responses(
        (status = 200, description = "Détails complets du livre", body = BookDto),
        (status = 404, description = "Livre introuvable"),
        (status = 500, description = "Erreur lors de la communication avec l'API Google Books")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "books"
)]
pub async fn get_book_by_id(
    State(state): State<Arc<AppState>>,
    Path(book_id): Path<String>,
) -> Result<Json<BookDto>, Json<Value>> {

    let resultat = get_by_id::get_book_by_id(
        &state.http_client,
        &state.api_url,
        &state.api_key,
        &book_id
    ).await;

    match resultat {
        Ok(data) => {
            if let Some(erreur_google) = data.get("error") {
                return Err(Json(json!({
                    "erreur": "Livre introuvable",
                    "details": erreur_google
                })));
            }

            let volume_info = &data["volumeInfo"];

            let id = data["id"].as_str().unwrap_or("").to_string();
            let title = volume_info["title"].as_str().unwrap_or("Titre inconnu").to_string();
            let description = volume_info["description"].as_str().map(|s| s.to_string());
            let maturity_rating = volume_info["maturityRating"].as_str().unwrap_or("").to_string();
            let published_date = volume_info["publishedDate"].as_str().unwrap_or("").to_string();

            let image_url = volume_info.get("imageLinks")
                .and_then(|links| links.get("thumbnail"))
                .and_then(|t| t.as_str())
                .map(|s| s.to_string());

            let page_count = volume_info["pageCount"].as_u64().map(Number::from).unwrap_or_else(|| Number::from(0));
            let average_rating = volume_info["averageRating"].as_f64().and_then(Number::from_f64).unwrap_or_else(|| Number::from(0));
            let ratings_count = volume_info["ratingsCount"].as_u64().map(Number::from).unwrap_or_else(|| Number::from(0));

            let authors = volume_info["authors"].as_array()
                .map(|auteurs_array| auteurs_array.iter().filter_map(|a| a.as_str()).map(|n| AuthorDto { name: n.to_string() }).collect())
                .unwrap_or_else(Vec::new);

            let categories = volume_info["categories"].as_array()
                .map(|categories_array| categories_array.iter().filter_map(|c| c.as_str()).flat_map(|c| c.split(" / ")).map(|n| CategoryDto { name: n.trim().to_string() }).collect())
                .unwrap_or_else(Vec::new);

            let book = BookDto {
                id, title, authors, description, image_url, page_count, categories, average_rating, ratings_count, maturity_rating, published_date,
            };

            Ok(Json(book))
        },
        Err(_) => Err(Json(json!({ "erreur": "Impossible de contacter l'API Google Books" })))
    }
}


#[utoipa::path(
    get,
    path = "/api/books/category/{id}",
    params(
        ("id" = i32, Path, description = "L'ID de la catégorie stockée dans la base de données locale"),
        ParametresRecherche // 👈 Récupère automatiquement tes paramètres (limit, offset)
    ),
    responses(
        (status = 200, description = "Liste des livres correspondant à la catégorie", body = BookSearchResponse),
        (status = 404, description = "Catégorie introuvable dans la base de données"),
        (status = 500, description = "Erreur interne ou de l'API Google Books")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "books"
)]
pub async fn get_books_by_category_id(
    State(state): State<Arc<AppState>>,
    Path(category_id): Path<i32>, // 👈 On récupère l'ID de l'URL
    Query(params): Query<ParametresRecherche>, // Pour gérer ?limit=10&offset=0
) -> Result<Json<BookSearchResponse>, Json<Value>> {

    // 1. On cherche le nom de la catégorie en base de données
    let category_name = match categories_service::get_category_name_by_id(&state.db_pool, category_id).await {
        Ok(Some(name)) => name,
        Ok(None) => return Err(Json(json!({ "erreur": "Catégorie introuvable en base de données" }))),
        Err(_) => return Err(Json(json!({ "erreur": "Erreur interne de la base de données" }))),
    };

    // 2. On formate la requête
    let raw_query = category_name;
    // On encode pour transformer les espaces en %20, etc.
    let google_query = urlencoding::encode(&raw_query).into_owned();

    println!("📡 Envoi à Google Books : URL={}, Query={}", state.api_url, google_query);

    let limit = params.limit.unwrap_or(10).min(40);
    let offset = params.offset.unwrap_or(0);

    let resultat = search_books::search_books(
        &state.http_client,
        &state.api_url,
        &state.api_key,
        &google_query,
        offset,
        limit
    ).await;

    // 3. On applique ton mapping habituel (identique à get_books_by_recherche)
    match resultat {
        Ok(data) => {
            let total_items = data.get("totalItems")
                .and_then(|t| t.as_u64())
                .map(|t| t as usize)
                .unwrap_or(0);

            let mut livres = Vec::new();

            if let Some(items) = data.get("items").and_then(|i| i.as_array()) {
                for item in items {
                    let volume_info = &item["volumeInfo"];

                    let id = item["id"].as_str().unwrap_or("").to_string();
                    let title = volume_info["title"].as_str().unwrap_or("Titre inconnu").to_string();
                    let description = volume_info["description"].as_str().map(|s| s.to_string());
                    let maturity_rating = volume_info["maturityRating"].as_str().unwrap_or("").to_string();
                    let published_date = volume_info["publishedDate"].as_str().unwrap_or("").to_string();

                    let image_url = volume_info.get("imageLinks")
                        .and_then(|links| links.get("thumbnail"))
                        .and_then(|t| t.as_str())
                        .map(|s| s.to_string());

                    let page_count = volume_info["pageCount"].as_u64().map(Number::from).unwrap_or_else(|| Number::from(0));
                    let average_rating = volume_info["averageRating"].as_f64().and_then(Number::from_f64).unwrap_or_else(|| Number::from(0));
                    let ratings_count = volume_info["ratingsCount"].as_u64().map(Number::from).unwrap_or_else(|| Number::from(0));

                    let authors = volume_info["authors"].as_array()
                        .map(|arr| arr.iter().filter_map(|a| a.as_str()).map(|n| AuthorDto { name: n.to_string() }).collect())
                        .unwrap_or_else(Vec::new);

                    let categories = volume_info["categories"].as_array()
                        .map(|arr| arr.iter().filter_map(|c| c.as_str()).flat_map(|c| c.split(" / ")).map(|n| CategoryDto { name: n.trim().to_string() }).collect())
                        .unwrap_or_else(Vec::new);

                    livres.push(BookDto {
                        id, title, authors, description, image_url, page_count, categories, average_rating, ratings_count, maturity_rating, published_date,
                    });
                }
            }

            Ok(Json(BookSearchResponse {
                pagination: PaginationDto { total_items, offset, limit },
                livres,
            }))
        },
        Err(_) => Err(Json(json!({ "erreur": "Impossible de contacter l'API Google Books" })))
    }
}