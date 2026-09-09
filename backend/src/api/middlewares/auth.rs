use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode, header::AUTHORIZATION},
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;
use crate::api::auth::jwt::Claims;

// Cette structure va "wrapper" nos claims.
// Si elle est présente dans les paramètres d'une route, la route est protégée.
pub struct RequireAuth(pub Claims);

impl<S> FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 1. Extraire le header "Authorization: Bearer <token>"
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .filter(|value| value.starts_with("Bearer "))
            .map(|value| &value[7..]);

        let token = auth_header.ok_or((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "erreur": "Token manquant ou mal formaté" })),
        ))?;

        // 2. Décoder et vérifier le token
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret_defaut".into());

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        ).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "erreur": "Token expiré ou invalide" })),
            )
        })?;

        // 3. Retourner les données de l'utilisateur (qui seront injectées dans la route)
        Ok(RequireAuth(token_data.claims))
    }
}


// Nouvelle structure pour les routes Admin
pub struct RequireAdmin(pub Claims);

impl<S> FromRequestParts<S> for RequireAdmin
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {

        // 1. On fait appel à RequireAuth pour valider et décoder le JWT
        let RequireAuth(claims) = RequireAuth::from_request_parts(parts, state).await?;

        // 2. On vérifie SIMPLEMENT la valeur du rôle contenu dans le token !
        // (Adapte "admin" si ton rôle s'appelle différemment, ex: "ADMIN" ou "Administrateur")
        if claims.role != "admin" {
            return Err((
                StatusCode::FORBIDDEN, // 403 Forbidden
                Json(json!({ "erreur": "Accès refusé. Droits administrateur requis." })),
            ));
        }

        // 3. Si c'est bien un admin, on laisse passer
        Ok(RequireAdmin(claims))
    }
}