use reqwest::Client;
use serde_json::Value;


// Ajoute cette fonction à la fin de ton fichier search_books.rs
pub async fn get_book_by_id(
    client: &Client, 
    base_url: &str, 
    api_key: &str, 
    book_id: &str
) -> Result<Value, reqwest::Error> {
    
    // L'URL de Google est : /volumes/ID_DU_LIVRE?key=CLE_API
    let url = format!("{}/volumes/{}?key={}", base_url, book_id, api_key);
    
    let reponse = client.get(&url).send().await?;
    reponse.json::<Value>().await
}