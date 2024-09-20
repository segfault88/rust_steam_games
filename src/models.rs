use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

// AppID,Name,Release date,Estimated owners,Peak CCU,Required age,Price,DiscountDLC count,About the game,Supported languages,Full audio languages,Reviews,Header image,Website,Support url,Support email,Windows,Mac,Linux,Metacritic score,Metacritic url,User score,Positive,Negative,Score rank,Achievements,Recommendations,Notes,Average playtime forever,Average playtime two weeks,Median playtime forever,Median playtime two weeks,Developers,Publishers,Categories,Genres,Tags,Screenshots,Movies

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    #[serde(rename = "AppID")]
    pub app_id: u64,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Release date")]
    release_date: String,
    #[serde(rename = "Estimated owners")]
    estimated_owners: String,
    #[serde(rename = "Peak CCU")]
    peak_ccu: u32,
    #[serde(rename = "Required age")]
    required_age: u8,
    #[serde(rename = "Price")]
    price: String,
    #[serde(rename = "DiscountDLC count")]
    discount_dlc_count: u32,
    #[serde(rename = "About the game")]
    about_the_game: String,
    #[serde(rename = "Supported languages")]
    supported_languages: String,
    #[serde(rename = "Full audio languages")]
    full_audio_languages: String,
    #[serde(rename = "Reviews")]
    reviews: String,
    #[serde(rename = "Header image")]
    header_image: String,
    #[serde(rename = "Website")]
    website: Option<String>,
    #[serde(rename = "Support url")]
    support_url: Option<String>,
    #[serde(rename = "Support email")]
    support_email: Option<String>,
    // #[serde(rename = "Windows")]
    // windows: String,
    // #[serde(rename = "Mac")]
    // mac: String,
    // #[serde(rename = "Linux")]
    // linux: String,
    // #[serde(rename = "Metacritic score")]
    // metacritic_score: Option<u8>,
    // #[serde(rename = "Metacritic url")]
    // metacritic_url: Option<String>,
    // #[serde(rename = "User score")]
    // user_score: Option<f32>,
    // #[serde(rename = "Positive")]
    // positive: u32,
    // #[serde(rename = "Negative")]
    // negative: u32,
    // #[serde(rename = "Score rank")]
    // score_rank: Option<u32>,
    // #[serde(rename = "Achievements")]
    // achievements: Option<u32>,
    // #[serde(rename = "Recommendations")]
    // recommendations: Option<u32>,
    // #[serde(rename = "Notes")]
    // notes: Option<String>,
    // #[serde(rename = "Average playtime forever")]
    // average_playtime_forever: u32,
    // #[serde(rename = "Average playtime two weeks")]
    // average_playtime_two_weeks: u32,
    // #[serde(rename = "Median playtime forever")]
    // median_playtime_forever: u32,
    // #[serde(rename = "Median playtime two weeks")]
    // median_playtime_two_weeks: u32,
    // #[serde(rename = "Developers")]
    // developers: String,
    // #[serde(rename = "Publishers")]
    // publishers: String,
    // #[serde(rename = "Categories")]
    // categories: String,
    // #[serde(rename = "Genres")]
    // genres: String,
    // #[serde(rename = "Tags")]
    // tags: String,
    // #[serde(rename = "Screenshots")]
    // screenshots: String,
    // #[serde(rename = "Movies")]
    // movies: String,
}
