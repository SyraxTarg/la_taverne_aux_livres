use std::collections::HashSet;
use std::sync::Arc;
use crate::AppState;

pub async fn creer_table_si_inexistante(db_pool: &sqlx::PgPool) {
    let query = "
        CREATE TABLE IF NOT EXISTS categories (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) UNIQUE NOT NULL
        );
    ";

    match sqlx::query(query).execute(db_pool).await {
        Ok(_) => println!("✅ Table 'categories' vérifiée ou créée avec succès."),
        Err(e) => eprintln!("❌ Erreur lors de la création de la table 'categories' : {}", e),
    }
}

pub async fn sauvegarder_nouvelles_categories(
    state: &Arc<AppState>,
    categories: HashSet<String>,
) {
    if categories.is_empty() {
        return;
    }

    println!("💾 [REPO] Sauvegarde de {} catégories en base...", categories.len());

    for categorie_nom in categories {
        let query = "
            INSERT INTO categories (name) 
            VALUES ($1) 
            ON CONFLICT (name) DO NOTHING
        ";

        let result = sqlx::query(query)
            .bind(&categorie_nom)
            .execute(&state.db_pool)
            .await;

        match result {
            Ok(_) => {
                println!(" - ✅ Catégorie traitée : {}", categorie_nom);
            }
            Err(e) => {
                eprintln!(" - ❌ Erreur pour la catégorie '{}' : {}", categorie_nom, e);
            }
        }
    }
}