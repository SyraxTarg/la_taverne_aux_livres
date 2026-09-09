use axum::{extract::State, Json};
use serde_json::{json, Value};
use std::sync::Arc;
use crate::AppState;
use crate::api::dto::requests::auth::{RegisterDto, LoginDto};
use crate::api::dto::responses::auth::LoginResponseDto;
use crate::api::auth::jwt::{hash_password, verify_password, create_jwt};
use validator::Validate;
use crate::api::service::users as service;

#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterDto,
    responses(
        (status = 200, description = "Utilisateur créé avec succès"),
        (status = 500, description = "Erreur lors du hachage ou de l'insertion en BDD")
    ),
    tag = "auth"
)]
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterDto>,
) -> Result<Json<Value>, Json<Value>> {

    if let Err(errors) = body.validate() {
        return Err(Json(json!({
            "erreur": "Données invalides",
            "details": errors.to_string()
        })));
    }


    let hashed_password = match hash_password(&body.password) {
        Ok(h) => h,
        Err(_) => return Err(Json(json!({ "erreur": "Erreur lors du hachage du mot de passe" }))),
    };

    match service::create_user(&state.db_pool, body.email, hashed_password, body.role_id).await {
        Ok(_) => Ok(Json(json!({ "message": "Utilisateur créé avec succès !" }))),
        Err(e) => Err(Json(json!({ "erreur": format!("Impossible de créer l'utilisateur : {}", e) }))),
    }
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginDto,
    responses(
        (status = 200, description = "Connexion réussie, retourne le jeton d'accès"),
        (status = 400, description = "Identifiants invalides"),
        (status = 500, description = "Erreur interne ou génération de token")
    ),
    tag = "auth"
)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginDto>,
) -> Result<Json<LoginResponseDto>, Json<Value>> {
    
    // 1. Récupération de l'utilisateur
    let user = service::find_by_email(&state.db_pool, &payload.email)
        .await
        .map_err(|_| Json(json!({ "erreur": "Erreur BDD" })))?;

    let user = match user {
        Some(u) => u,
        None => return Err(Json(json!({ "erreur": "Identifiants invalides" }))),
    };

    // 2. Vérification du mot de passe
    if !verify_password(&payload.password, &user.password) {
        return Err(Json(json!({ "erreur": "Identifiants invalides" })));
    }

    // 3. Récupération du rôle (on ouvre le Result, puis le Option)
    let role_opt = crate::api::service::roles::find_role_by_user_email(&state.db_pool, &user.email)
        .await
        .map_err(|_| Json(json!({ "erreur": "Erreur lors de la récupération du rôle" })))?;

    let role_name = match role_opt {
        Some(r) => r.name, // On extrait la String `name`
        None => return Err(Json(json!({ "erreur": "Aucun rôle assigné à cet utilisateur" }))),
    };

    // 4. Création du JWT
    // (Ajoute un & devant role_name si ta fonction create_jwt attend un &str plutôt qu'une String)
    let token = create_jwt(&user.email, &role_name).map_err(|_| {
        Json(json!({ "erreur": "Erreur lors de la génération du token" }))
    })?;

    Ok(Json(LoginResponseDto { access_token: token }))
}