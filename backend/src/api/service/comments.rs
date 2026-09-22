use crate::api::{
    dto::responses::comment::{CommentResponseDto, CommentWithRepliesDto, UserCommentResponseDto},
    entities::{comment, user},
    repo::comments as repo,
};
use chrono;
use sea_orm::DatabaseConnection;
use sea_orm::*;
use std::collections::HashMap;

pub async fn creer_table_si_inexistante(db: &DatabaseConnection) {
    repo::creer_table_si_inexistante(db).await
}

pub async fn create_comment(
    db: &DatabaseConnection,
    content: String,
    user_id: i32,
    book_id: String,
    parent_id: Option<i32>,
) -> Result<comment::Model, DbErr> {
    if let Some(pid) = parent_id {
        let parent_comment = comment::Entity::find_by_id(pid).one(db).await?;

        match parent_comment {
            Some(parent) => {
                if parent.book_id != book_id {
                    return Err(DbErr::Custom(
                        "Impossible de répondre à un commentaire d'un autre livre.".to_string(),
                    ));
                }
                if parent.parent_id.is_some() {
                    return Err(DbErr::Custom(
                        "Impossible de répondre à une réponse. Vous ne pouvez répondre qu'à un commentaire principal.".to_string(),
                    ));
                }
            }
            None => {
                return Err(DbErr::Custom(
                    "Le commentaire parent spécifié n'existe pas.".to_string(),
                ));
            }
        }
    }

    let new_comment = comment::ActiveModel {
        content: Set(content),
        user_id: Set(user_id),
        book_id: Set(book_id),
        parent_id: Set(parent_id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };

    repo::insert_comment(db, new_comment).await
}

pub async fn get_comments_by_book_id(
    db: &DatabaseConnection,
    book_id: &str,
) -> Result<Vec<(comment::Model, Option<user::Model>)>, DbErr> {
    repo::get_comments_by_book_id(db, book_id).await
}

#[derive(Clone, Debug)]
pub struct CommentWithUser {
    pub comment: comment::Model,
    pub user: Option<user::Model>,
}

impl std::ops::Deref for CommentWithUser {
    type Target = comment::Model;
    fn deref(&self) -> &Self::Target {
        &self.comment
    }
}

impl From<(comment::Model, Option<user::Model>)> for CommentWithUser {
    fn from((comment, user): (comment::Model, Option<user::Model>)) -> Self {
        Self { comment, user }
    }
}

impl CommentWithUser {
    pub fn to_dto(&self) -> CommentResponseDto {
        let user = match &self.user {
            Some(u) => UserCommentResponseDto {
                id: u.id,
                email: u.email.clone(),
            },
            None => UserCommentResponseDto {
                id: self.comment.user_id,
                email: "Utilisateur inconnu".to_string(),
            },
        };

        CommentResponseDto {
            id: self.comment.id,
            content: self.comment.content.clone(),
            user,
            book_id: self.comment.book_id.clone(),
            parent_id: self.comment.parent_id,
            created_at: self.comment.created_at.to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CommentTree {
    pub original: CommentWithUser,
    pub replies: Vec<CommentWithUser>,
}

impl CommentTree {
    pub fn to_dto(self) -> CommentWithRepliesDto {
        let user = match &self.original.user {
            Some(u) => UserCommentResponseDto {
                id: u.id,
                email: u.email.clone(),
            },
            None => UserCommentResponseDto {
                id: self.original.comment.user_id,
                email: "Utilisateur inconnu".to_string(),
            },
        };

        CommentWithRepliesDto {
            id: self.original.comment.id,
            content: self.original.comment.content,
            user,
            book_id: self.original.comment.book_id,
            created_at: self.original.comment.created_at.to_string(),
            reponses: self.replies.into_iter().map(|rep| rep.to_dto()).collect(),
        }
    }
}

pub async fn get_comments_structured(
    db: &DatabaseConnection,
    book_id: &str,
) -> Result<Vec<CommentTree>, sea_orm::DbErr> {
    let commentaires = repo::get_comments_by_book_id(db, book_id).await?;

    let mut originals = Vec::new();
    let mut map_reponses: HashMap<i32, Vec<CommentWithUser>> = HashMap::new();

    for item in commentaires {
        let item = CommentWithUser::from(item);
        if let Some(parent_id) = item.parent_id {
            map_reponses.entry(parent_id).or_default().push(item);
        } else {
            originals.push(item);
        }
    }

    let mut resultat_final = Vec::new();
    for orig in originals {
        let reponses_associees = map_reponses.remove(&orig.id).unwrap_or_default();

        resultat_final.push(CommentTree {
            original: orig,
            replies: reponses_associees,
        });
    }

    Ok(resultat_final)
}

#[derive(Debug)]
pub enum CommentServiceError {
    NotFound,
    Forbidden(String),
    BadRequest(String),
    Database(DbErr),
}

impl From<DbErr> for CommentServiceError {
    fn from(err: DbErr) -> Self {
        CommentServiceError::Database(err)
    }
}

pub async fn get_comment_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<CommentTree>, DbErr> {
    let (comment, user) = match repo::get_comment_by_id(db, id).await? {
        Some(c) => c,
        None => return Ok(None),
    };

    let replies = repo::get_replies_by_parent_id(db, id).await?;

    Ok(Some(CommentTree {
        original: CommentWithUser { comment, user },
        replies: replies.into_iter().map(CommentWithUser::from).collect(),
    }))
}

pub async fn update_comment(
    db: &DatabaseConnection,
    id: i32,
    user_id: i32,
    new_content: String,
) -> Result<comment::Model, CommentServiceError> {
    if new_content.trim().is_empty() {
        return Err(CommentServiceError::BadRequest(
            "Le contenu du commentaire ne peut pas être vide.".to_string(),
        ));
    }

    let (existing, _) = repo::get_comment_by_id(db, id)
        .await?
        .ok_or(CommentServiceError::NotFound)?;

    // Seul l'auteur peut modifier son propre commentaire
    if existing.user_id != user_id {
        return Err(CommentServiceError::Forbidden(
            "Vous ne pouvez modifier que vos propres commentaires.".to_string(),
        ));
    }

    let updated = repo::update_comment(db, id, new_content).await?;
    Ok(updated)
}

pub async fn delete_comment(
    db: &DatabaseConnection,
    id: i32,
    user_id: i32,
) -> Result<u64, CommentServiceError> {
    let (existing, _) = repo::get_comment_by_id(db, id)
        .await?
        .ok_or(CommentServiceError::NotFound)?;

    // Seul l'auteur peut supprimer son propre commentaire
    if existing.user_id != user_id {
        return Err(CommentServiceError::Forbidden(
            "Vous ne pouvez supprimer que vos propres commentaires.".to_string(),
        ));
    }

    let deleted_count = repo::delete_comment_and_replies(db, id).await?;
    Ok(deleted_count)
}
