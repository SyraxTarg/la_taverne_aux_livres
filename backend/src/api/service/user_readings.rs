use crate::api::entities::user_reading;
use crate::api::repo::user_reading as repo;
use sea_orm::*;

pub async fn creer_table_si_inexistante(db: &DatabaseConnection) {
    repo::creer_table_si_inexistante(db).await;
    println!("✅ Table 'users_reaidng' vérifiée via l'ORM.");
}

pub async fn create_user_reading(
    db: &DatabaseConnection,
    user_id: i32,
    book_id: String,
    note: Option<i32>,
) -> Result<user_reading::Model, DbErr> {
    if let Some(existing) = repo::find_by_user_and_book(db, user_id, &book_id).await? {
        repo::update_user_reading(db, existing, note).await
    } else {
        let new_reading = user_reading::ActiveModel {
            user_id: Set(user_id),
            book_id: Set(book_id),
            note: Set(note),
            read_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };
        repo::insert_user_reading(db, new_reading).await
    }
}

pub async fn get_readings_by_user_id(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Vec<user_reading::Model>, DbErr> {
    repo::get_readings_by_user_id(db, user_id).await
}

pub async fn delete_user_reading(
    db: &DatabaseConnection,
    user_id: i32,
    book_id: &str,
) -> Result<bool, DbErr> {
    let rows = repo::delete_by_user_and_book(db, user_id, book_id).await?;
    Ok(rows > 0)
}

pub async fn get_book_ratings_stats(
    db: &DatabaseConnection,
    book_id: &str,
) -> Result<(Option<f64>, u64), DbErr> {
    repo::get_ratings_stats_by_book_id(db, book_id).await
}
