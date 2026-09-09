use sea_orm::*;
use crate::api::entities::user;

pub async fn creer_table_si_inexistante(db: &DatabaseConnection) {
    let builder = db.get_database_backend();
    let schema = sea_orm::Schema::new(builder);
    
    let stmt = schema.create_table_from_entity(user::Entity)
        .if_not_exists()
        .to_owned();
    
    let _ = db.execute(&stmt).await;
}

pub async fn insert_user(
    db: &DatabaseConnection,
    new_user: user::ActiveModel
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