use std::collections::HashSet;
use std::sync::Arc;
use crate::AppState;
use crate::api::entities::category;
use sea_orm::*;


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
    state: &Arc<AppState>,
    categories: HashSet<String>,
) {
    if categories.is_empty() {
        return;
    }

    println!("💾 [ORM] Sauvegarde de {} catégories...", categories.len());

    for categorie_nom in categories {
        // On crée un "ActiveModel" pour l'insertion
        let new_category = category::ActiveModel {
            name: Set(categorie_nom.clone()),
            ..Default::default() // L'ID s'incrémente tout seul
        };

        // On insère, et on ignore si la catégorie existe déjà (grâce à la contrainte UNIQUE)
        match new_category.insert(&state.db_pool).await {
            Ok(_) => println!(" - ✅ Inséré : {}", categorie_nom),
            Err(_) => {
                // Erreur de duplication (la catégorie existe déjà), on l'ignore calmement
            }
        }
    }
}

// Recherche du nom par ID avec l'ORM
pub async fn get_category_name_by_id(
    db: &DatabaseConnection, 
    category_id: i32
) -> Result<Option<String>, DbErr> {
    
    // Équivalent propre de "SELECT name FROM categories WHERE id = ?"
    let cat = category::Entity::find_by_id(category_id)
        .one(db)
        .await?;

    Ok(cat.map(|c| c.name))
}