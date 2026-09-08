use reqwest::Client;
use serde_json::Value;

pub async fn search_books(
    client: &Client, 
    base_url: &str, 
    api_key: &str, 
    recherche: &str,
    start_index: usize,
    max_results: usize
) -> Result<Value, reqwest::Error> {
    
    let url = format!("{}/volumes", base_url);
    
    // On crée un tableau avec tous nos paramètres convertis en String
    // C'est beaucoup plus propre pour Rust et ça gère les espaces tout seul !
    let query_params = [
        ("q", recherche.to_string()),
        ("key", api_key.to_string()),
        ("startIndex", start_index.to_string()),
        ("maxResults", max_results.to_string()),
    ];

    let reponse = client.get(&url)
        .query(&query_params)
        .send()
        .await?;
        
    reponse.json::<Value>().await
}