use serde_json::Number;
use std::collections::{HashMap, HashSet};

use crate::AppState;
use crate::api::dto::responses::author::AuthorDto;
use crate::api::dto::responses::book::BookDto;
use crate::api::dto::responses::categories::CategoryDto;
use crate::api::dto::responses::recommendation::RecommendedBookDto;
use crate::api::repo::user_reading as user_reading_repo;
use crate::proxy;

pub fn compute_cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}

pub fn parse_book_dto(item: &serde_json::Value) -> Option<BookDto> {
    let volume_info = &item["volumeInfo"];
    let id = item["id"].as_str().unwrap_or("").to_string();
    if id.is_empty() {
        return None;
    }

    let title = volume_info["title"]
        .as_str()
        .unwrap_or("Titre inconnu")
        .to_string();
    let description = volume_info["description"].as_str().map(|s| s.to_string());
    let maturity_rating = volume_info["maturityRating"]
        .as_str()
        .unwrap_or("")
        .to_string();
    let published_date = volume_info["publishedDate"]
        .as_str()
        .unwrap_or("")
        .to_string();

    let image_url = volume_info
        .get("imageLinks")
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
                .map(|nom| AuthorDto {
                    name: nom.to_string(),
                })
                .collect()
        })
        .unwrap_or_else(Vec::new);

    let categories = volume_info["categories"]
        .as_array()
        .map(|categories_array| {
            categories_array
                .iter()
                .filter_map(|cat| cat.as_str())
                .flat_map(|cat_str| cat_str.split(" / "))
                .map(|nom| CategoryDto {
                    name: nom.trim().to_string(),
                })
                .collect()
        })
        .unwrap_or_else(Vec::new);

    Some(BookDto {
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
    })
}

fn is_vague_or_generic_category(cat: &str) -> bool {
    let lower = cat.trim().to_lowercase();
    matches!(
        lower.as_str(),
        "" | "general"
            | "reference"
            | "literary collections"
            | "unclassifiable"
            | "miscellaneous"
            | "fiction"
            | "non-fiction"
            | "nonfiction"
            | "juvenile fiction"
            | "juvenile nonfiction"
            | "young adult fiction"
            | "général"
            | "générale"
            | "ouvrages de référence"
            | "divers"
            | "autre"
            | "autres"
            | "books"
            | "livres"
    )
}

fn liked_book_to_query_text(book: &BookDto) -> String {
    let authors_str = book
        .authors
        .iter()
        .map(|a| a.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let specific_cats: Vec<_> = book
        .categories
        .iter()
        .map(|c| c.name.as_str())
        .filter(|c| !is_vague_or_generic_category(c))
        .collect();
    let categories_str = specific_cats.join(", ");
    let desc = book.description.as_deref().unwrap_or("");
    format!(
        "query: Titre: {} - Auteurs: {} - Genres: {} - Description: {}",
        book.title, authors_str, categories_str, desc
    )
}

fn candidate_to_passage_text(book: &BookDto) -> String {
    let authors_str = book
        .authors
        .iter()
        .map(|a| a.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let specific_cats: Vec<_> = book
        .categories
        .iter()
        .map(|c| c.name.as_str())
        .filter(|c| !is_vague_or_generic_category(c))
        .collect();
    let categories_str = specific_cats.join(", ");
    let desc = book.description.as_deref().unwrap_or("");
    format!(
        "passage: Titre: {} - Auteurs: {} - Genres: {} - Description: {}",
        book.title, authors_str, categories_str, desc
    )
}

/// Génère les recommandations personnalisées pour un utilisateur connecté
pub async fn get_recommendations_for_user(
    state: &AppState,
    user_id: i32,
    limit: usize,
) -> Result<Vec<RecommendedBookDto>, String> {
    // 1. Récupération des lectures de l'utilisateur
    let readings = user_reading_repo::get_readings_by_user_id(&state.db_pool, user_id)
        .await
        .map_err(|e| format!("Erreur BDD: {:?}", e))?;

    if readings.is_empty() {
        return Ok(Vec::new());
    }

    // Garde en mémoire les IDs des livres déjà lus pour ne pas les re-recommander
    let read_book_ids: HashSet<String> = readings.iter().map(|r| r.book_id.clone()).collect();

    // 2. Sélection des lectures notées :
    // Seuls les livres explicitement notés avec une note >= 3 sont pris en compte pour définir les goûts
    let mut liked_readings: Vec<_> = readings
        .into_iter()
        .filter(|r| r.note.map_or(false, |n| n >= 3))
        .collect();

    if liked_readings.is_empty() {
        return Ok(Vec::new());
    }

    liked_readings.sort_by(|a, b| b.note.unwrap_or(0).cmp(&a.note.unwrap_or(0)));
    let top_liked: Vec<_> = liked_readings.into_iter().take(5).collect();

    // 3. Récupération des détails des livres aimés via l'API Google Books
    let mut liked_books = Vec::new();
    let mut favorite_categories = HashSet::new();
    let mut read_book_titles = HashSet::new();

    for reading in &top_liked {
        if let Ok(data) = proxy::get_by_id::get_book_by_id(
            &state.http_client,
            &state.api_url,
            &state.api_key,
            &reading.book_id,
        )
        .await
        {
            if let Some(book) = parse_book_dto(&data) {
                read_book_titles.insert(book.title.trim().to_lowercase());
                for cat in &book.categories {
                    if !is_vague_or_generic_category(&cat.name) {
                        favorite_categories.insert(cat.name.clone());
                    }
                }
                liked_books.push((book, reading.note.unwrap_or(3)));
            }
        }
    }

    if liked_books.is_empty() {
        return Ok(Vec::new());
    }

    // 4. Construction des requêtes candidates ciblées
    let mut candidate_queries = Vec::new();

    // a) Genres spécifiques non vagues
    for cat in &favorite_categories {
        let cat_query = format!("subject:\"{}\"", cat);
        if !candidate_queries.contains(&cat_query) {
            candidate_queries.push(cat_query);
        }
    }

    // b) Auteurs des livres aimés
    for (book, _) in &liked_books {
        if let Some(author) = book.authors.first() {
            let author_query = format!("inauthor:\"{}\"", author.name);
            if !candidate_queries.contains(&author_query) {
                candidate_queries.push(author_query);
            }
        }
    }

    // c) Mots-clés des titres si le pool de requêtes est limité
    if candidate_queries.len() < 3 {
        for (book, _) in &liked_books {
            let keywords: Vec<&str> = book
                .title
                .split_whitespace()
                .filter(|w| w.len() > 3 && !w.chars().all(|c| c.is_ascii_punctuation()))
                .take(3)
                .collect();
            if !keywords.is_empty() {
                let title_query = keywords.join(" ");
                if !candidate_queries.contains(&title_query) {
                    candidate_queries.push(title_query);
                }
            }
        }
    }

    // Fallback de sécurité si aucune requête générée
    let queries: Vec<String> = if candidate_queries.is_empty() {
        liked_books
            .iter()
            .map(|(b, _)| b.title.clone())
            .take(3)
            .collect()
    } else {
        candidate_queries.into_iter().take(6).collect()
    };

    // 5. Exécution parallèle des requêtes candidats
    let mut candidate_futures = Vec::new();
    for query in &queries {
        candidate_futures.push(proxy::search_books::search_books(
            &state.http_client,
            &state.api_url,
            &state.api_key,
            query,
            0,
            20,
        ));
    }

    let search_results = futures::future::join_all(candidate_futures).await;

    let mut candidate_books: HashMap<String, BookDto> = HashMap::new();
    let mut candidate_titles: HashSet<String> = HashSet::new();

    for res in search_results {
        if let Ok(data) = res {
            if let Some(items) = data.get("items").and_then(|i| i.as_array()) {
                for item in items {
                    if let Some(book) = parse_book_dto(item) {
                        let normalized_title = book.title.trim().to_lowercase();
                        // Évite les livres déjà lus (ID ou titre similaire) et les doublons de candidats
                        if !read_book_ids.contains(&book.id)
                            && !read_book_titles.contains(&normalized_title)
                            && !candidate_titles.contains(&normalized_title)
                        {
                            candidate_titles.insert(normalized_title);
                            candidate_books.entry(book.id.clone()).or_insert(book);
                        }
                    }
                }
            }
        }
    }

    if candidate_books.is_empty() {
        return Ok(Vec::new());
    }

    // 6. Filtrage et priorisation des candidats :
    // On sépare les candidats ayant une vraie description de ceux qui n'en ont pas
    let (with_desc, without_desc): (Vec<BookDto>, Vec<BookDto>) =
        candidate_books.into_values().partition(|b| {
            b.description
                .as_ref()
                .map_or(false, |d| !d.trim().is_empty())
        });

    // Si on a suffisamment de candidats avec description (>= limit), on garde uniquement ceux-ci
    // pour garantir une qualité sémantique optimale et des résultats riches.
    // Sinon, on complète avec les livres sans description.
    let candidate_list: Vec<BookDto> = if with_desc.len() >= limit {
        with_desc
    } else {
        let mut combined = with_desc;
        combined.extend(without_desc);
        combined
    };

    if candidate_list.is_empty() {
        return Ok(Vec::new());
    }

    // 7. Calcul de l'embedding du profil de l'utilisateur (barycentre pondéré avec préfixe "query: ")
    let liked_texts: Vec<String> = liked_books
        .iter()
        .map(|(b, _)| liked_book_to_query_text(b))
        .collect();
    let liked_embeddings = state
        .embedding_model
        .embed(liked_texts, None)
        .map_err(|e| format!("Erreur FastEmbed: {:?}", e))?;

    if liked_embeddings.is_empty() {
        return Ok(Vec::new());
    }

    let emb_dim = liked_embeddings[0].len();
    let mut user_vector = vec![0.0f32; emb_dim];
    let mut total_weight = 0.0f32;

    for (emb, (_, note)) in liked_embeddings.iter().zip(&liked_books) {
        let weight = *note as f32;
        for i in 0..emb_dim {
            user_vector[i] += emb[i] * weight;
        }
        total_weight += weight;
    }

    if total_weight > 0.0 {
        for i in 0..emb_dim {
            user_vector[i] /= total_weight;
        }
    }

    // 8. Vectorisation des candidats (préfixe "passage: ") et calcul de la similarité cosinus
    let candidate_texts: Vec<String> = candidate_list
        .iter()
        .map(candidate_to_passage_text)
        .collect();
    let candidate_embeddings = state
        .embedding_model
        .embed(candidate_texts, None)
        .map_err(|e| format!("Erreur FastEmbed candidats: {:?}", e))?;

    let mut scored_books = Vec::new();
    for (book, emb) in candidate_list.into_iter().zip(candidate_embeddings) {
        let score = compute_cosine_similarity(&user_vector, &emb);
        let rounded_score = (score * 100.0).round() / 100.0;
        scored_books.push(RecommendedBookDto {
            book,
            similarity_score: rounded_score,
        });
    }

    // 9. Tri par score décroissant et sélection des N meilleurs
    scored_books.sort_by(|a, b| {
        b.similarity_score
            .partial_cmp(&a.similarity_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    scored_books.truncate(limit);
    Ok(scored_books)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_cosine_similarity_identical() {
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![1.0, 2.0, 3.0];
        let sim = compute_cosine_similarity(&v1, &v2);
        assert!((sim - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_cosine_similarity_proportional() {
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![2.0, 4.0, 6.0];
        let sim = compute_cosine_similarity(&v1, &v2);
        assert!((sim - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let v1 = vec![1.0, 0.0];
        let v2 = vec![0.0, 1.0];
        let sim = compute_cosine_similarity(&v1, &v2);
        assert_eq!(sim, 0.0);
    }

    #[test]
    fn test_cosine_similarity_opposite() {
        let v1 = vec![1.0, 0.0];
        let v2 = vec![-1.0, 0.0];
        let sim = compute_cosine_similarity(&v1, &v2);
        assert!((sim - (-1.0)).abs() < 1e-5);
    }

    #[test]
    fn test_cosine_similarity_zero_vector() {
        let v1 = vec![0.0, 0.0, 0.0];
        let v2 = vec![1.0, 2.0, 3.0];
        let sim = compute_cosine_similarity(&v1, &v2);
        assert_eq!(sim, 0.0);
    }

    #[test]
    fn test_is_vague_or_generic_category() {
        // Doit identifier les catégories vagues
        assert!(is_vague_or_generic_category("General"));
        assert!(is_vague_or_generic_category("general"));
        assert!(is_vague_or_generic_category("Reference"));
        assert!(is_vague_or_generic_category("Literary Collections"));
        assert!(is_vague_or_generic_category("Fiction"));
        assert!(is_vague_or_generic_category("non-fiction"));
        assert!(is_vague_or_generic_category("juvenile fiction"));
        assert!(is_vague_or_generic_category("général"));
        assert!(is_vague_or_generic_category(""));
        assert!(is_vague_or_generic_category("   "));

        // Ne doit pas filtrer les catégories spécifiques
        assert!(!is_vague_or_generic_category("Science Fiction"));
        assert!(!is_vague_or_generic_category("Dystopian"));
        assert!(!is_vague_or_generic_category("Fantasy"));
        assert!(!is_vague_or_generic_category("History"));
        assert!(!is_vague_or_generic_category("Philosophy"));
    }

    #[test]
    fn test_parse_book_dto_valid() {
        let payload = json!({
            "id": "book_abc_123",
            "volumeInfo": {
                "title": "Dune",
                "authors": ["Frank Herbert"],
                "description": "L'épopée de Paul Atréides sur Arrakis.",
                "pageCount": 412,
                "categories": ["Fiction / Science Fiction"],
                "averageRating": 4.5,
                "ratingsCount": 120,
                "imageLinks": {
                    "thumbnail": "http://example.com/dune.jpg"
                },
                "publishedDate": "1965",
                "maturityRating": "NOT_MATURE"
            }
        });

        let parsed = parse_book_dto(&payload).expect("Le parsing de BookDto doit réussir");
        assert_eq!(parsed.id, "book_abc_123");
        assert_eq!(parsed.title, "Dune");
        assert_eq!(parsed.authors.len(), 1);
        assert_eq!(parsed.authors[0].name, "Frank Herbert");
        assert_eq!(
            parsed.description,
            Some("L'épopée de Paul Atréides sur Arrakis.".to_string())
        );
        assert_eq!(
            parsed.image_url,
            Some("http://example.com/dune.jpg".to_string())
        );
        // Categories "Fiction / Science Fiction" sont séparées
        assert_eq!(parsed.categories.len(), 2);
        assert_eq!(parsed.categories[0].name, "Fiction");
        assert_eq!(parsed.categories[1].name, "Science Fiction");
    }

    #[test]
    fn test_parse_book_dto_empty_id() {
        let payload = json!({
            "id": "",
            "volumeInfo": {
                "title": "Sans ID"
            }
        });
        assert!(parse_book_dto(&payload).is_none());
    }

    #[test]
    fn test_parse_book_dto_defaults() {
        let payload = json!({
            "id": "minimal_id",
            "volumeInfo": {
                "title": "Livre Minimal"
            }
        });

        let parsed = parse_book_dto(&payload).expect("Parsing doit réussir");
        assert_eq!(parsed.id, "minimal_id");
        assert_eq!(parsed.title, "Livre Minimal");
        assert!(parsed.authors.is_empty());
        assert!(parsed.categories.is_empty());
        assert_eq!(parsed.description, None);
        assert_eq!(parsed.image_url, None);
    }

    #[test]
    fn test_liked_book_to_query_text() {
        let book = BookDto {
            id: "1".into(),
            title: "1984".into(),
            authors: vec![AuthorDto {
                name: "George Orwell".into(),
            }],
            categories: vec![
                CategoryDto {
                    name: "Fiction".into(),
                }, // vague, doit être filtrée
                CategoryDto {
                    name: "Dystopian".into(),
                }, // spécifique, conservée
            ],
            description: Some("Big Brother is watching you.".into()),
            image_url: None,
            page_count: serde_json::Number::from(328),
            average_rating: serde_json::Number::from_f64(4.8).unwrap(),
            ratings_count: serde_json::Number::from(500),
            maturity_rating: "NOT_MATURE".into(),
            published_date: "1949".into(),
        };

        let query_text = liked_book_to_query_text(&book);
        assert!(query_text.starts_with("query: "));
        assert!(query_text.contains("1984"));
        assert!(query_text.contains("George Orwell"));
        assert!(query_text.contains("Dystopian"));
        assert!(!query_text.contains("Fiction"));
    }

    #[test]
    fn test_candidate_to_passage_text() {
        let book = BookDto {
            id: "2".into(),
            title: "Le Meilleur des mondes".into(),
            authors: vec![AuthorDto {
                name: "Aldous Huxley".into(),
            }],
            categories: vec![CategoryDto {
                name: "Science Fiction".into(),
            }],
            description: Some("Société dystopique et clonage.".into()),
            image_url: None,
            page_count: serde_json::Number::from(285),
            average_rating: serde_json::Number::from_f64(4.6).unwrap(),
            ratings_count: serde_json::Number::from(300),
            maturity_rating: "NOT_MATURE".into(),
            published_date: "1932".into(),
        };

        let passage_text = candidate_to_passage_text(&book);
        assert!(passage_text.starts_with("passage: "));
        assert!(passage_text.contains("Le Meilleur des mondes"));
        assert!(passage_text.contains("Aldous Huxley"));
        assert!(passage_text.contains("Science Fiction"));
    }
}
