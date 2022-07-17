use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Response<T> {
    data: Vec<ResponseData<T>>,
}

#[derive(Deserialize, Debug)]
struct ResponseData<T> {
    attributes: T,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Image {
    tiny: String,
    small: String,
    medium: String,
    large: String,
    original: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SearchResult {
    slug: String,
    subtype: String,
    titles: HashMap<String, String>,
    #[serde(rename(deserialize = "posterImage"))]
    poster_image: Image,
}

type SearchResponse = Response<SearchResult>;

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
