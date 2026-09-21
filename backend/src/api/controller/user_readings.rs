use crate::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::{Value, json}; // 👈 Ajout pour les messages d'erreur
use std::sync::Arc;

use crate::api::dto::requests::user_reading::UserReadingDto;
use crate::api::dto::responses::user_reading::UserReadingResponseDto;
use crate::api::middlewares::auth::RequireAuth;
use crate::api::service::user_readings as service;
use crate::api::service::users as users_service;

// Cette route est protégée !
#[utoipa::path(
    put,
    path = "/api/readings",
    responses(
        (status = 200, description = "Lecture enregistrée ou mise à jour", body = UserReadingResponseDto),
        (status = 400, description = "Requête invalide (note hors de l'intervalle [1, 5])"),
        (status = 401, description = "Non autorisé"),
        (status = 404, description = "Utilisateur introuvable"),
        (status = 500, description = "Erreur interne du serveur")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "lectures"
)]
pub async fn create_reading(
    RequireAuth(claims): RequireAuth,
    State(state): State<Arc<AppState>>,
    Json(body): Json<UserReadingDto>,
) -> Result<(StatusCode, Json<UserReadingResponseDto>), (StatusCode, Json<Value>)> {
    // 1. Validation de la note si renseignée
    if let Some(note) = body.note {
        if !(1..=5).contains(&note) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "erreur": "La note doit être comprise entre 1 et 5" })),
            ));
        }
    }

    let user = match users_service::find_by_email(&state.db_pool, &claims.sub).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({ "erreur": "Utilisateur introuvable" })),
            ));
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "erreur": "Erreur interne de la base de données" })),
            ));
        }
    };

    let new_reading =
        service::create_user_reading(&state.db_pool, user.id.into(), body.book_id, body.note).await;

    match new_reading {
        Ok(reading) => {
            let response = UserReadingResponseDto {
                id: reading.id,
                user_id: reading.user_id,
                book_id: reading.book_id,
                note: reading.note,
                read_at: reading.read_at.to_string(),
            };

            // Renvoie un code 200 (OK)
            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            let error_message = e.to_string();

            // Sinon, c'est une vraie erreur technique (panne de BDD, etc.) -> 500
            println!("❌ Erreur technique ajout de lecture : {:?}", error_message);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "erreur": "Impossible de sauvegarder la lecture" })),
            ))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/users/{user_id}/readings",
    responses(
        (status = 200, description = "Liste des lectures de l'utilisateur", body = [UserReadingResponseDto]),
        (status = 500, description = "Erreur interne du serveur")
    ),
    params(
        ("user_id" = i32, Path, description = "L'identifiant de l'utilisateur")
    ),
    tag = "lectures"
)]
pub async fn get_readings_by_user_id(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<UserReadingResponseDto>>, (StatusCode, Json<Value>)> {
    match service::get_readings_by_user_id(&state.db_pool, user_id).await {
        Ok(readings) => {
            let response = readings
                .into_iter()
                .map(|r| UserReadingResponseDto {
                    id: r.id,
                    user_id: r.user_id,
                    book_id: r.book_id,
                    note: r.note,
                    read_at: r.read_at.to_string(),
                })
                .collect();

            Ok(Json(response))
        }
        Err(e) => {
            println!("❌ Erreur technique récupération des lectures : {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "erreur": "Impossible de récupérer les lectures" })),
            ))
        }
    }
}

#[utoipa::path(
    delete,
    path = "/api/readings/{book_id}",
    responses(
        (status = 200, description = "Lecture supprimée avec succès"),
        (status = 401, description = "Non autorisé (Token manquant ou invalide)"),
        (status = 404, description = "Lecture introuvable pour ce livre"),
        (status = 500, description = "Erreur interne du serveur")
    ),
    params(
        ("book_id" = String, Path, description = "L'identifiant Google Books du livre à retirer des lectures")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "lectures"
)]
pub async fn delete_reading(
    RequireAuth(claims): RequireAuth,
    State(state): State<Arc<AppState>>,
    Path(book_id): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match service::delete_user_reading(&state.db_pool, claims.id, &book_id).await {
        Ok(true) => Ok(Json(json!({ "message": "Lecture supprimée avec succès" }))),
        Ok(false) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "erreur": "Lecture introuvable pour ce livre" })),
        )),
        Err(e) => {
            println!("❌ Erreur technique suppression de lecture : {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "erreur": "Impossible de supprimer la lecture" })),
            ))
        }
    }
}
