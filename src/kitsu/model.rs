use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response<T> {
    pub data: Vec<ResponseData<T>>,
    pub included: Option<Vec<Included>>,
}

#[derive(Deserialize, Debug)]
pub struct ResponseData<T> {
    pub attributes: T,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Included {
    #[serde(rename = "genres")]
    Genre { attributes: GenreInfo },
    #[serde(rename = "characters")]
    Character { attributes: CharacterInfo },
    #[serde(rename = "streamingLinks")]
    StreamingLink { attributes: StreamingLinkInfo },
    #[serde(rename = "producers")]
    Producer { attributes: ProducerInfo },
    #[serde(rename = "episodes")]
    Episode { attributes: EpisodeInfo },
    #[serde(rename = "mediaCharacters")]
    MediaCharacter,
    #[serde(rename = "mediaProductions")]
    MediaProduction,
    #[serde(rename = "animeProductions")]
    AnimeProduction,
}

#[derive(Deserialize, Debug)]
pub struct GenreInfo {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct EpisodeInfo {
    #[serde(rename = "canonicalTitle")]
    pub title: Option<String>,
    #[serde(rename = "relativeNumber")]
    pub episode_no: Option<u32>,
    #[serde(rename = "seasonNumber")]
    pub season_no: Option<u32>,
    pub length: Option<u32>,
    #[serde(rename = "airdate")]
    pub aired_on: Option<NaiveDate>,
    pub thumbnail: Option<Image>,
    pub synopsis: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CharacterInfo {
    pub name: String,
    pub description: Option<String>,
    pub image: Image,
}

#[derive(Deserialize, Debug)]
pub struct StreamingLinkInfo {
    pub url: String,
    pub subs: Option<Vec<String>>,
    pub dubs: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct ProducerInfo {
    pub name: String,
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

#[derive(Deserialize, Debug, Clone)]
pub struct AnimeInfo {
    pub slug: String,
    pub subtype: AnimeType,
    pub status: AnimeStatus,
    #[serde(rename = "canonicalTitle")]
    pub canonical_title: String,
    pub titles: HashMap<String, String>,
    #[serde(rename = "updatedAt")]
    pub last_updated: DateTime<Utc>,
    #[serde(rename = "startDate")]
    pub start_date: NaiveDate,
    #[serde(rename = "endDate")]
    pub end_date: NaiveDate,
    pub synopsis: Option<String>,
    #[serde(rename = "posterImage")]
    pub poster_image: Image,
    #[serde(rename = "ageRatingGuide")]
    pub age_rating: Option<String>,
    #[serde(rename = "youtubeVideoId")]
    pub youtube_ref: Option<String>,
    pub nsfw: bool,
    #[serde(rename = "episodeLength")]
    pub episode_length: Option<u32>,
    #[serde(rename = "totalLength")]
    pub total_length: Option<u32>,
}

pub type SearchResponse = Response<SearchResult>;
pub type AnimeResponse = Response<AnimeInfo>;

#[derive(Deserialize, Debug, Clone)]
pub enum AnimeStatus {
    #[serde(rename = "current")]
    Current,
    #[serde(rename = "finished")]
    Finished,
    #[serde(rename = "unreleased")]
    Unreleased,
    #[serde(rename = "upcoming")]
    Upcoming,
    #[serde(rename = "tba")]
    ToBeAnnounced,
}

#[derive(Deserialize, Debug, Clone)]
pub enum AnimeType {
    #[serde(rename = "ona")]
    OriginalNetAnimation,
    #[serde(rename = "ova")]
    OriginalVideoAnimation,
    #[serde(rename = "tv")]
    TelevisionShow,
    #[serde(rename = "movie")]
    Movie,
    #[serde(rename = "music")]
    Music,
    #[serde(rename = "special")]
    Special,
}
