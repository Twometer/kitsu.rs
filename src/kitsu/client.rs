use super::model::{SearchResponse, SearchResult};

pub async fn search(query: &str) -> anyhow::Result<Vec<SearchResult>> {
    let query_encoded = urlencoding::encode(query);
    let search_url = format!("https://kitsu.io/api/edge/anime?filter[text]={}&fields[anime]=slug,titles,posterImage,subtype", query_encoded);
    let results = reqwest::get(search_url)
        .await?
        .json::<SearchResponse>()
        .await?
        .data
        .iter()
        .map(|e| e.attributes.clone())
        .collect::<Vec<SearchResult>>();
    Ok(results)
}
