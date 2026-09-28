use crate::api::entities::user;
use sea_orm::*;

pub async fn creer_table_si_inexistante(db: &DatabaseConnection) {
    let builder = db.get_database_backend();
    let schema = sea_orm::Schema::new(builder);

    let stmt = schema
        .create_table_from_entity(user::Entity)
        .if_not_exists()
        .to_owned();

    let _ = db.execute(&stmt).await;

    // S'assurer que la colonne 'username' est présente si la table existait déjà
    let _ = db
        .execute_unprepared(
            "ALTER TABLE users ADD COLUMN IF NOT EXISTS username VARCHAR(255) NOT NULL DEFAULT '';",
        )
        .await;
}

pub async fn insert_user(
    db: &DatabaseConnection,
    new_user: user::ActiveModel,
) -> Result<user::Model, DbErr> {
    new_user.insert(db).await
}

pub async fn find_by_email(
    db: &DatabaseConnection,
    email: &str,
) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await
}

pub async fn find_by_id(db: &DatabaseConnection, id: &i32) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find()
        .filter(user::Column::Id.eq(id))
        .one(db)
        .await
}
