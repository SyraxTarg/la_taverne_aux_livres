use sea_orm::*;
use crate::api::entities::{role, user};

pub async fn creer_table_et_roles_base(db: &DatabaseConnection) -> Result<(), DbErr> {
    let builder = db.get_database_backend();
    let schema = sea_orm::Schema::new(builder);

    let stmt = schema.create_table_from_entity(role::Entity)
        .if_not_exists()
        .to_owned();

    db.execute(&stmt).await?;
    println!("✅ Table 'roles' vérifiée via l'ORM.");

    // Insérer les rôles de base (admin, viewer) s'ils n'existent pas déjà
    let roles_de_base = vec!["admin", "viewer"];

    for nom_role in roles_de_base {
        let existe = role::Entity::find()
            .filter(role::Column::Name.eq(nom_role))
            .one(db)
            .await?;

        if existe.is_none() {
            let nouveau_role = role::ActiveModel {
                name: Set(nom_role.to_string()),
                ..Default::default()
            };
            nouveau_role.insert(db).await?;
            println!("🌱 Rôle par défaut inséré : {}", nom_role);
        }
    }

    Ok(())
}


pub async fn find_by_role_name(
    db: &DatabaseConnection,
    role: &str,
) -> Result<Option<role::Model>, DbErr> {
    role::Entity::find()
        .filter(role::Column::Name.eq(role))
        .one(db)
        .await
}


pub async fn find_role_by_user_email(
    db: &DatabaseConnection,
    email: &str,
) -> Result<Option<role::Model>, DbErr> {

    let utilisateur = user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await?;

    if let Some(u) = utilisateur {
        let role_trouve = role::Entity::find_by_id(u.role_id)
            .one(db)
            .await?;

        Ok(role_trouve)
    } else {
        Ok(None)
    }
}