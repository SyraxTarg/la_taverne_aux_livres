use axum::{Json, extract::State, http::StatusCode};
use serde_json::{Value, json};
use std::sync::Arc;

use crate::AppState;
use crate::api::dto::responses::recommendation::RecommendedBookDto;
use crate::api::middlewares::auth::RequireAuth;
use crate::api::service::recommandations as service;

#[utoipa::path(
    get,
    path = "/api/recommandations",
    responses(
        (status = 200, description = "Liste des livres recommandés personnalisés pour l'utilisateur connecté", body = [RecommendedBookDto]),
        (status = 401, description = "Non autorisé (Token JWT manquant ou invalide)"),
        (status = 500, description = "Erreur interne du serveur lors de la génération des recommandations")
    ),
    tag = "recommandations",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_recommendations(
    RequireAuth(claims): RequireAuth,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<RecommendedBookDto>>, (StatusCode, Json<Value>)> {
    match service::get_recommendations_for_user(&state, claims.id, 10).await {
        Ok(recommendations) => Ok(Json(recommendations)),
        Err(e) => {
            println!("❌ Erreur génération des recommandations : {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "erreur": "Impossible de générer les recommandations" })),
            ))
        }
    }
}
