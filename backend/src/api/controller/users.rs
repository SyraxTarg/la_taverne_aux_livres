use axum::{
    extract::{State},
    Json
};
use serde_json::{json, Value}; // 👈 Ajout pour les messages d'erreur
use std::sync::Arc;
use crate::AppState;

use crate::api::middlewares::auth::RequireAuth;
use crate::api::dto::responses::user::UserResponseDto;
use crate::api::dto::responses::role::RoleResponseDto;
use crate::api::service::users as service;

// Cette route est protégée !
#[utoipa::path(
    get,
    path = "/api/users/me",
    responses(
        (status = 200, description = "Profil récupéré"),
        (status = 401, description = "Non autorisé"),
        (status = 404, description = "Utilisateur introuvable")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "utilisateurs"
)]
pub async fn get_me(
    RequireAuth(claims): RequireAuth,
    State(state): State<Arc<AppState>>, // 👈 Récupération propre de la BDD via le state
) -> Result<Json<UserResponseDto>, Json<Value>> { // 👈 Json<Value> pour l'erreur

    let user = match service::find_by_email(&state.db_pool, &claims.sub).await {
        Ok(Some(u)) => u,
        Ok(None) => return Err(Json(json!({ "erreur": "Utilisateur introuvable" }))),
        Err(_) => return Err(Json(json!({ "erreur": "Erreur interne de la base de données" }))),
    };

    let response = UserResponseDto {
        id: user.id.into(),
        email: user.email,
        role: RoleResponseDto {
            id: user.role_id.into(),
            role: claims.role,
        }
    };

    Ok(Json(response))
}