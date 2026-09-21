use crate::api::entities::user_reading;
use sea_orm::*;

pub async fn creer_table_si_inexistante(db: &DatabaseConnection) {
    let builder = db.get_database_backend();
    let schema = sea_orm::Schema::new(builder);

    let stmt = schema
        .create_table_from_entity(user_reading::Entity)
        .if_not_exists()
        .to_owned();

    let _ = db.execute(&stmt).await;
}

pub async fn insert_user_reading(
    db: &DatabaseConnection,
    new_reading: user_reading::ActiveModel,
) -> Result<user_reading::Model, DbErr> {
    new_reading.insert(db).await
}

pub async fn find_by_user_and_book(
    db: &DatabaseConnection,
    user_id: i32,
    book_id: &str,
) -> Result<Option<user_reading::Model>, DbErr> {
    user_reading::Entity::find()
        .filter(user_reading::Column::UserId.eq(user_id))
        .filter(user_reading::Column::BookId.eq(book_id))
        .one(db)
        .await
}

pub async fn update_user_reading(
    db: &DatabaseConnection,
    existing: user_reading::Model,
    note: Option<i32>,
) -> Result<user_reading::Model, DbErr> {
    let mut active: user_reading::ActiveModel = existing.into();
    active.note = Set(note);
    active.read_at = Set(chrono::Utc::now().naive_utc());
    active.update(db).await
}

pub async fn get_readings_by_user_id(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Vec<user_reading::Model>, DbErr> {
    user_reading::Entity::find()
        .filter(user_reading::Column::UserId.eq(user_id))
        .order_by_desc(user_reading::Column::ReadAt)
        .all(db)
        .await
}

pub async fn delete_by_user_and_book(
    db: &DatabaseConnection,
    user_id: i32,
    book_id: &str,
) -> Result<u64, DbErr> {
    let res = user_reading::Entity::delete_many()
        .filter(user_reading::Column::UserId.eq(user_id))
        .filter(user_reading::Column::BookId.eq(book_id))
        .exec(db)
        .await?;
    Ok(res.rows_affected)
}
