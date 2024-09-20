use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "20200")]
    pub games: HashMap<String, Game>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub name: String,
    #[serde(rename = "release_date")]
    pub release_date: String,
    #[serde(rename = "required_age")]
    pub required_age: i64,
    pub price: f64,
    #[serde(rename = "dlc_count")]
    pub dlc_count: i64,
    #[serde(rename = "detailed_description")]
    pub detailed_description: String,
    #[serde(rename = "about_the_game")]
    pub about_the_game: String,
    #[serde(rename = "short_description")]
    pub short_description: String,
    pub reviews: String,
    #[serde(rename = "header_image")]
    pub header_image: String,
    pub website: String,
    #[serde(rename = "support_url")]
    pub support_url: String,
    #[serde(rename = "support_email")]
    pub support_email: String,
    pub windows: bool,
    pub mac: bool,
    pub linux: bool,
    #[serde(rename = "metacritic_score")]
    pub metacritic_score: i64,
    #[serde(rename = "metacritic_url")]
    pub metacritic_url: String,
    pub achievements: i64,
    pub recommendations: i64,
    pub notes: String,
    #[serde(rename = "supported_languages")]
    pub supported_languages: Vec<String>,
    #[serde(rename = "full_audio_languages")]
    pub full_audio_languages: Vec<Value>,
    pub packages: Vec<Package>,
    pub developers: Vec<String>,
    pub publishers: Vec<String>,
    pub categories: Vec<String>,
    pub genres: Vec<String>,
    pub screenshots: Vec<String>,
    pub movies: Vec<String>,
    #[serde(rename = "user_score")]
    pub user_score: i64,
    #[serde(rename = "score_rank")]
    pub score_rank: String,
    pub positive: i64,
    pub negative: i64,
    #[serde(rename = "estimated_owners")]
    pub estimated_owners: String,
    #[serde(rename = "average_playtime_forever")]
    pub average_playtime_forever: i64,
    #[serde(rename = "average_playtime_2weeks")]
    pub average_playtime_2weeks: i64,
    #[serde(rename = "median_playtime_forever")]
    pub median_playtime_forever: i64,
    #[serde(rename = "median_playtime_2weeks")]
    pub median_playtime_2weeks: i64,
    #[serde(rename = "peak_ccu")]
    pub peak_ccu: i64,
    pub tags: Tags,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    pub title: String,
    pub description: String,
    pub subs: Vec<Sub>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sub {
    pub text: String,
    pub description: String,
    pub price: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags {
    #[serde(rename = "Indie")]
    pub indie: i64,
    #[serde(rename = "Casual")]
    pub casual: i64,
    #[serde(rename = "Sports")]
    pub sports: i64,
    #[serde(rename = "Bowling")]
    pub bowling: i64,
}


fn main() -> Result<(), Box<dyn Error>> {
    let f = std::fs::OpenOptions::new().read(true).open("data/games.json")?;
    
    let _games = serde_json::from_reader(f)?;

    Ok(())
}
