use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response<T> {
    pub data: Vec<ResponseData<T>>,
}

#[derive(Deserialize, Debug)]
pub struct ResponseData<T> {
    pub attributes: T,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Image {
    pub tiny: String,
    pub small: String,
    pub medium: String,
    pub large: String,
    pub original: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SearchResult {
    pub slug: String,
    pub subtype: String,
    pub titles: HashMap<String, String>,
    #[serde(rename(deserialize = "posterImage"))]
    pub poster_image: Image,
}

pub type SearchResponse = Response<SearchResult>;

pub enum AnimeStatus {
    Airing,
    Finished,
    Unreleased,
    Upcoming,
    ToBeAnnounced,
}

pub enum AnimeType {
    OriginalNetAnimation,
    OriginalVideoAnimation,
    TelevisionShow,
    Movie,
    Music,
    Special,
}
