use serde_derive::Deserialize;
use serde_derive::Serialize;

// AppID,Name,Release date,Estimated owners,Peak CCU,Required age,Price,DiscountDLC count,About the game,Supported languages,Full audio languages,Reviews,Header image,Website,Support url,Support email,Windows,Mac,Linux,Metacritic score,Metacritic url,User score,Positive,Negative,Score rank,Achievements,Recommendations,Notes,Average playtime forever,Average playtime two weeks,Median playtime forever,Median playtime two weeks,Developers,Publishers,Categories,Genres,Tags,Screenshots,Movies

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    #[serde(rename = "AppID")]
    pub app_id: i64,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Release date")]
    pub release_date: String,
    #[serde(rename = "Estimated owners")]
    pub estimated_owners: String,
    #[serde(rename = "Peak CCU")]
    pub peak_ccu: u32,
    #[serde(rename = "Required age")]
    pub required_age: u8,
    #[serde(rename = "Price")]
    pub price: String,
    #[serde(rename = "DiscountDLC count")]
    pub discount_dlc_count: u32,
    #[serde(rename = "Mystery")]
    pub mystery: i64,
    #[serde(rename = "About the game")]
    pub about_the_game: String,
    #[serde(rename = "Supported languages")]
    pub supported_languages: String,
    #[serde(rename = "Full audio languages")]
    pub full_audio_languages: String,
    #[serde(rename = "Reviews")]
    pub reviews: String,
    #[serde(rename = "Header image")]
    pub header_image: String,
    #[serde(rename = "Website")]
    pub website: Option<String>,
    #[serde(rename = "Support url")]
    pub support_url: Option<String>,
    #[serde(rename = "Support email")]
    pub support_email: Option<String>,
    #[serde(rename = "Windows")]
    pub windows: String,
    #[serde(rename = "Mac")]
    pub mac: String,
    #[serde(rename = "Linux")]
    pub linux: String,
    #[serde(rename = "Metacritic score")]
    pub metacritic_score: Option<u8>,
    #[serde(rename = "Metacritic url")]
    pub metacritic_url: Option<String>,
    #[serde(rename = "User score")]
    pub user_score: Option<f32>,
    #[serde(rename = "Positive")]
    pub positive: u32,
    #[serde(rename = "Negative")]
    pub negative: u32,
    #[serde(rename = "Score rank")]
    pub score_rank: Option<f64>,
    #[serde(rename = "Achievements")]
    pub achievements: Option<u32>,
    #[serde(rename = "Recommendations")]
    pub recommendations: Option<u32>,
    #[serde(rename = "Notes")]
    pub notes: Option<String>,
    #[serde(rename = "Average playtime forever")]
    pub average_playtime_forever: u32,
    #[serde(rename = "Average playtime two weeks")]
    pub average_playtime_two_weeks: u32,
    #[serde(rename = "Median playtime forever")]
    pub median_playtime_forever: u32,
    #[serde(rename = "Median playtime two weeks")]
    pub median_playtime_two_weeks: u32,
    #[serde(rename = "Developers")]
    pub developers: String,
    #[serde(rename = "Publishers")]
    pub publishers: String,
    #[serde(rename = "Categories")]
    pub categories: String,
    #[serde(rename = "Genres")]
    pub genres: String,
    #[serde(rename = "Tags")]
    pub tags: String,
    #[serde(rename = "Screenshots")]
    pub screenshots: String,
    #[serde(rename = "Movies")]
    pub movies: String,
}
