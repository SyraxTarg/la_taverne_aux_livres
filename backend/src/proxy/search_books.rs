pub async fn search_books(
    client: &reqwest::Client, // 👈 Ajout de & ici
    api_url: &str,
    api_key: &str,
    search_query: &str,
    offset: usize,
    limit: usize,
) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!(
        "{}/volumes?q={}&startIndex={}&maxResults={}&key={}",
        api_url, search_query, offset, limit, api_key
    );

    let reponse = client.get(&url)
        .send()
        .await?;

    let data = reponse.json::<serde_json::Value>().await?;
    Ok(data)
}