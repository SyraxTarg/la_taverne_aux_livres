use sea_orm::*;
use crate::api::entities::user;
use crate::api::repo::users;

pub async fn creer_table_si_inexistante(db: &DatabaseConnection) {
    users::creer_table_si_inexistante(db).await;
    println!("✅ Table 'users' vérifiée via l'ORM.");
}

pub async fn create_user(
    db: &DatabaseConnection,
    email: String,
    hashed_password: String,
    role_id: i32,
) -> Result<user::Model, DbErr> {
    let new_user = user::ActiveModel {
        email: Set(email),
        password: Set(hashed_password),
        role_id: Set(role_id),
        ..Default::default()
    };
    users::insert_user(db, new_user).await
}

pub async fn find_by_email(
    db: &DatabaseConnection,
    email: &str,
) -> Result<Option<user::Model>, DbErr> {
    users::find_by_email(db, email).await
}