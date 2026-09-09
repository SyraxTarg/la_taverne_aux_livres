use crate::api::entities::comment;
use sea_orm::*;


pub async fn creer_table_si_inexistante(db: &DatabaseConnection) {
    let builder = db.get_database_backend();
    let schema = sea_orm::Schema::new(builder);

    let stmt = schema.create_table_from_entity(comment::Entity)
        .if_not_exists()
        .to_owned();

    let _ = db.execute(&stmt).await;
    println!("✅ Table 'comments' vérifiée via l'ORM.");
}

pub async fn insert_comment(
    db: &DatabaseConnection,
    new_comment: comment::ActiveModel
) -> Result<comment::Model, DbErr> {
    new_comment.insert(db).await
}

pub async fn get_comments_by_book_id(
    db: &DatabaseConnection,
    book_id: &str,
) -> Result<Vec<comment::Model>, DbErr> {
    comment::Entity::find()
        .filter(comment::Column::BookId.eq(book_id))
        .order_by_desc(comment::Column::CreatedAt) // Les plus récents en premier
        .all(db)
        .await
}