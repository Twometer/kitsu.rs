use model::{AnimeResponse, SearchResponse, SearchResult};

pub mod error;
pub mod model;

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

pub async fn get_anime(slug: &str) -> anyhow::Result<AnimeResponse> {
    let slug = slug.trim().to_lowercase();
    let slug_encoded = urlencoding::encode(&slug);
    let anime_url = format!("https://kitsu.io/api/edge/anime?filter[slug]={}&include=genres,productions.company,animeProductions.producer,episodes,streamingLinks,characters.character", slug_encoded);
    let result = reqwest::get(anime_url)
        .await?
        .json::<AnimeResponse>()
        .await?;

    Ok(result)
}
