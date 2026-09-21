use crate::api::{entities::comment, repo::comments as repo};
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
) -> Result<Vec<comment::Model>, DbErr> {
    repo::get_comments_by_book_id(db, book_id).await
}

pub struct CommentTree {
    pub original: comment::Model,
    pub replies: Vec<comment::Model>,
}

pub async fn get_comments_structured(
    db: &DatabaseConnection,
    book_id: &str,
) -> Result<Vec<CommentTree>, sea_orm::DbErr> {
    let commentaires = repo::get_comments_by_book_id(db, book_id).await?;

    let mut originals = Vec::new();
    let mut map_reponses: HashMap<i32, Vec<comment::Model>> = HashMap::new();

    for c in commentaires {
        if let Some(parent_id) = c.parent_id {
            map_reponses.entry(parent_id).or_default().push(c);
        } else {
            originals.push(c);
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
    let comment = match repo::get_comment_by_id(db, id).await? {
        Some(c) => c,
        None => return Ok(None),
    };

    let replies = repo::get_replies_by_parent_id(db, id).await?;

    Ok(Some(CommentTree {
        original: comment,
        replies,
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

    let existing = repo::get_comment_by_id(db, id)
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
    let existing = repo::get_comment_by_id(db, id)
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
