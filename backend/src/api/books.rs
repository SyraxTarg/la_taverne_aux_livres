use axum::{extract::State, Json};
use serde_json::{json, Value};
use std::sync::Arc;

// On importe l'état depuis main.rs et la fonction depuis notre proxy
use crate::AppState;
use crate::proxy::find_volumes;

pub async fn get_books(State(state): State<Arc<AppState>>) -> Json<Value> {
    let recherche = "fondation asimov";
    
    // On appelle notre proxy de manière propre
    let resultat = find_volumes::find_volumes(
        &state.http_client, 
        &state.api_url, 
        &state.api_key, 
        recherche
    ).await;

    // Gestion du retour
    match resultat {
        Ok(data) => Json(data),
        Err(_) => Json(json!({ "erreur": "Impossible de contacter l'API Google Books" })),
    }
}