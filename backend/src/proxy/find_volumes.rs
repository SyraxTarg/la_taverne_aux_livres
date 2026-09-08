use reqwest::Client;
use serde_json::Value;

// Fonction réutilisable pour appeler Google Books
pub async fn find_volumes(
    client: &Client,
    base_url: &str,
    api_key: &str,
    recherche: &str
) -> Result<Value, reqwest::Error> {

    let url = format!("{}/volumes?q={}&key={}", base_url, recherche, api_key);

    // On envoie la requête et on parse le JSON
    let reponse = client.get(&url).send().await?;
    reponse.json::<Value>().await
}