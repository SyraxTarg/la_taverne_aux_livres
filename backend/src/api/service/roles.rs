use sea_orm::*;
use crate::api::repo::roles as repo;
use crate::api::entities::{role};

pub async fn creer_table_et_roles_base(db: &DatabaseConnection) -> Result<(), DbErr> {
    repo::creer_table_et_roles_base(db).await
}



pub async fn find_role_by_user_email(
    db: &DatabaseConnection,
    email: &str,
) -> Result<Option<role::Model>, DbErr> {
    repo::find_role_by_user_email(db, email).await
}