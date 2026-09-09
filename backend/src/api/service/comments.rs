use sea_orm::*;
use chrono;
use crate::api::{entities::comment, repo::comments as repo};


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