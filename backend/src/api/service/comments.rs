use sea_orm::*;
use chrono;
use crate::api::{entities::comment, repo::comments as repo};
use sea_orm::DatabaseConnection;
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
        let parent_comment = comment::Entity::find_by_id(pid)
            .one(db)
            .await?;

        match parent_comment {
            Some(parent) => {
                if parent.book_id != book_id {
                    return Err(DbErr::Custom(
                        "Impossible de répondre à un commentaire d'un autre livre.".to_string(),
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