use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use std::sync::Arc;

// N'oublie pas d'adapter ces imports à l'architecture de ton projet
use crate::AppState;
use crate::api::middlewares::auth::RequireAuth;
use crate::api::service::comments as comments_service; // Ton service
use crate::api::dto::requests::comment::CreateCommentDto;
use crate::api::dto::responses::comment::CommentResponseDto;

#[utoipa::path(
    post,
    path = "/api/comments",
    request_body = CreateCommentDto,
    responses(
        (status = 201, description = "Commentaire créé avec succès", body = CommentResponseDto),
        (status = 400, description = "Requête invalide (ex: contenu vide)"),
        (status = 401, description = "Non autorisé (Token manquant ou invalide)"),
        (status = 500, description = "Erreur interne du serveur")
    ),
    tag = "comments",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_comment(
    State(state): State<Arc<AppState>>,
    RequireAuth(claims): RequireAuth, // 👈 Bloque la route si pas connecté
    Json(payload): Json<CreateCommentDto>,
) -> Result<(StatusCode, Json<CommentResponseDto>), (StatusCode, Json<Value>)> {
    
    // 1. Petite validation basique (tu peux aussi utiliser la crate `validator` ici)
    if payload.content.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "erreur": "Le contenu du commentaire ne peut pas être vide" }))
        ));
    }

    // 2. Appel de ton service
    // ⚠️ Remplace `claims.id` par le nom exact du champ ID dans ta structure Claims
    let resultat = comments_service::create_comment(
        &state.db_pool,
        payload.content,
        claims.id, // L'ID de l'auteur provient du token, c'est ultra-sécurisé !
        payload.book_id,
        payload.parent_id,
    ).await;

    // 3. Gestion de la réponse
    match resultat {
        Ok(comment) => {
            let response = CommentResponseDto {
                id: comment.id,
                content: comment.content,
                user_id: comment.user_id,
                book_id: comment.book_id,
                parent_id: comment.parent_id,
                created_at: comment.created_at.to_string(), // Convertit la date en String pour le JSON
            };
            
            // Renvoie un code 201 (Created)
            Ok((StatusCode::CREATED, Json(response)))
        },
        Err(e) => {
            let error_message = e.to_string();

            // Si c'est l'une de nos erreurs de validation métier, on renvoie un 400 Bad Request propre
            if error_message.contains("Impossible de répondre") || error_message.contains("Le commentaire parent") {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "erreur": error_message }))
                ));
            }

            // Sinon, c'est une vraie erreur technique (panne de BDD, etc.) -> 500
            println!("❌ Erreur technique création commentaire : {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "erreur": "Impossible de sauvegarder le commentaire" }))
            ))
        }
    }
}