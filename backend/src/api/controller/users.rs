use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::{json, Value}; // 👈 Ajout pour les messages d'erreur
use std::sync::Arc;
use crate::AppState;

use crate::api::middlewares::auth::RequireAuth;
use crate::api::dto::responses::user::{UserResponseDto, UserSimpleResponseDto};
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


#[utoipa::path(
    get,
    path = "/{user_id}",
    responses(
        (status = 200, description = "Profil récupéré"),
        (status = 404, description = "Utilisateur introuvable")
    ),
    params(
        ("user_id" = i32, Path, description = "L'identifiant de l'utilisateur")
    ),
    tag = "utilisateurs"
)]
pub async fn get_user_by_id(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserSimpleResponseDto>, Json<Value>> {

    let user = match service::find_by_id(&state.db_pool, &user_id).await {
        Ok(Some(u)) => u,
        Ok(None) => return Err(Json(json!({ "erreur": "Utilisateur introuvable" }))),
        Err(_) => return Err(Json(json!({ "erreur": "Erreur interne de la base de données" }))),
    };

    let response = UserSimpleResponseDto {
        id: user.id.into(),
        email: user.email
    };

    Ok(Json(response))
}