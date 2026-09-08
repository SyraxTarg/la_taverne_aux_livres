use axum::{
    extract::{State, Query, Path},
    Json
};
use serde::Deserialize;
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

#[derive(Deserialize)]
pub struct ParametresRecherche {
    pub recherche: Option<String>,
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

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
            crate::api::repo::categories::sauvegarder_nouvelles_categories(
                &state,
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