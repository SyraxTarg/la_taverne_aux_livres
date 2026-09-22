use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::{Value, json};
use std::sync::Arc;

// N'oublie pas d'adapter ces imports à l'architecture de ton projet
use crate::AppState;
use crate::api::dto::requests::comment::{CreateCommentDto, UpdateCommentDto};
use crate::api::dto::responses::comment::{CommentResponseDto, CommentWithRepliesDto, UserCommentResponseDto};
use crate::api::middlewares::auth::RequireAuth;
use crate::api::service::comments as comments_service; // Ton service
use crate::api::service::comments::CommentServiceError;

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
            Json(json!({ "erreur": "Le contenu du commentaire ne peut pas être vide" })),
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
    )
    .await;

    // 3. Gestion de la réponse
    match resultat {
        Ok(comment) => {
            let response = CommentResponseDto {
                id: comment.id,
                content: comment.content,
                user: UserCommentResponseDto {
                    id: claims.id,
                    email: claims.sub.clone(),
                },
                book_id: comment.book_id,
                parent_id: comment.parent_id,
                created_at: comment.created_at.to_string(), // Convertit la date en String pour le JSON
            };

            // Renvoie un code 201 (Created)
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => {
            let error_message = e.to_string();

            // Si c'est l'une de nos erreurs de validation métier, on renvoie un 400 Bad Request propre
            if error_message.contains("Impossible de répondre")
                || error_message.contains("Le commentaire parent")
            {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "erreur": error_message })),
                ));
            }

            // Sinon, c'est une vraie erreur technique (panne de BDD, etc.) -> 500
            println!("❌ Erreur technique création commentaire : {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "erreur": "Impossible de sauvegarder le commentaire" })),
            ))
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/comments/{id}",
    responses(
        (status = 200, description = "Commentaire récupéré avec succès", body = CommentWithRepliesDto),
        (status = 404, description = "Commentaire introuvable"),
        (status = 500, description = "Erreur interne du serveur")
    ),
    params(
        ("id" = i32, Path, description = "L'identifiant du commentaire")
    ),
    tag = "comments"
)]
pub async fn get_comment_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<CommentWithRepliesDto>, (StatusCode, Json<Value>)> {
    match comments_service::get_comment_by_id(&state.db_pool, id).await {
        Ok(Some(tree)) => Ok(Json(tree.to_dto())),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "erreur": "Commentaire introuvable" })),
        )),
        Err(e) => {
            println!("❌ Erreur récupération commentaire {}: {:?}", id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "erreur": "Impossible de récupérer le commentaire" })),
            ))
        }
    }
}

#[utoipa::path(
    put,
    path = "/api/comments/{id}",
    request_body = UpdateCommentDto,
    responses(
        (status = 200, description = "Commentaire mis à jour avec succès", body = CommentResponseDto),
        (status = 400, description = "Requête invalide (ex: contenu vide)"),
        (status = 401, description = "Non autorisé (Token manquant ou invalide)"),
        (status = 403, description = "Accès refusé (Vous ne pouvez modifier que vos propres commentaires)"),
        (status = 404, description = "Commentaire introuvable"),
        (status = 500, description = "Erreur interne du serveur")
    ),
    params(
        ("id" = i32, Path, description = "L'identifiant du commentaire à modifier")
    ),
    tag = "comments",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_comment(
    State(state): State<Arc<AppState>>,
    RequireAuth(claims): RequireAuth,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateCommentDto>,
) -> Result<Json<CommentResponseDto>, (StatusCode, Json<Value>)> {
    match comments_service::update_comment(&state.db_pool, id, claims.id, payload.content).await {
        Ok(comment) => {
            let response = CommentResponseDto {
                id: comment.id,
                content: comment.content,
                user: UserCommentResponseDto {
                    id: claims.id,
                    email: claims.sub.clone(),
                },
                book_id: comment.book_id,
                parent_id: comment.parent_id,
                created_at: comment.created_at.to_string(),
            };
            Ok(Json(response))
        }
        Err(CommentServiceError::NotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "erreur": "Commentaire introuvable" })),
        )),
        Err(CommentServiceError::Forbidden(msg)) => {
            Err((StatusCode::FORBIDDEN, Json(json!({ "erreur": msg }))))
        }
        Err(CommentServiceError::BadRequest(msg)) => {
            Err((StatusCode::BAD_REQUEST, Json(json!({ "erreur": msg }))))
        }
        Err(CommentServiceError::Database(e)) => {
            println!(
                "❌ Erreur technique mise à jour commentaire {}: {:?}",
                id, e
            );
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "erreur": "Impossible de mettre à jour le commentaire" })),
            ))
        }
    }
}

#[utoipa::path(
    delete,
    path = "/api/comments/{id}",
    responses(
        (status = 200, description = "Commentaire et ses réponses supprimés avec succès"),
        (status = 401, description = "Non autorisé (Token manquant ou invalide)"),
        (status = 403, description = "Accès refusé (Vous ne pouvez supprimer que vos propres commentaires)"),
        (status = 404, description = "Commentaire introuvable"),
        (status = 500, description = "Erreur interne du serveur")
    ),
    params(
        ("id" = i32, Path, description = "L'identifiant du commentaire à supprimer")
    ),
    tag = "comments",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_comment(
    State(state): State<Arc<AppState>>,
    RequireAuth(claims): RequireAuth,
    Path(id): Path<i32>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match comments_service::delete_comment(&state.db_pool, id, claims.id).await {
        Ok(deleted_count) => Ok(Json(json!({
            "message": "Commentaire et toutes ses réponses supprimés avec succès",
            "nombre_supprimes": deleted_count
        }))),
        Err(CommentServiceError::NotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "erreur": "Commentaire introuvable" })),
        )),
        Err(CommentServiceError::Forbidden(msg)) => {
            Err((StatusCode::FORBIDDEN, Json(json!({ "erreur": msg }))))
        }
        Err(CommentServiceError::BadRequest(msg)) => {
            Err((StatusCode::BAD_REQUEST, Json(json!({ "erreur": msg }))))
        }
        Err(CommentServiceError::Database(e)) => {
            println!(
                "❌ Erreur technique suppression commentaire {}: {:?}",
                id, e
            );
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "erreur": "Impossible de supprimer le commentaire" })),
            ))
        }
    }
}
