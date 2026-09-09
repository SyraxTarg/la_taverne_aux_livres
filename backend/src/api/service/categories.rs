use std::collections::HashSet;
use crate::api::entities::category;
use sea_orm::*;
use crate::api::repo::categories as repo;


pub async fn creer_table_si_inexistante(db: &DatabaseConnection) {
    let builder = db.get_database_backend();
    let schema = sea_orm::Schema::new(builder);

    let stmt = schema.create_table_from_entity(category::Entity)
        .if_not_exists()
        .to_owned();

    let _ = db.execute(&stmt).await;
    println!("✅ Table 'categories' vérifiée via l'ORM.");
}


// Sauvegarde intelligente avec l'ORM (ActiveModel)
pub async fn sauvegarder_nouvelles_categories(
    db: &DatabaseConnection,
    categories: HashSet<String>,
) {
    if categories.is_empty() {
        return;
    }
    println!("💾 [ORM] Sauvegarde de {} catégories...", categories.len());

    for categorie_nom in categories {
        repo::insert_category(db, categorie_nom).await
    }
}

// Recherche du nom par ID avec l'ORM
pub async fn get_category_name_by_id(
    db: &DatabaseConnection,
    category_id: i32
) -> Result<Option<String>, DbErr> {
    println!("🔍 Recherche de la catégorie avec l'ID : {}", category_id);

    repo::get_category_name_by_id(db, category_id).await
}