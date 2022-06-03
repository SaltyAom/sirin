use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Hentai {
    pub id: u32,
    // pub title: String,
    // pub tags: Vec<String>,
    // pub page: u16,
}

#[derive(Deserialize)]
pub struct MeilisearchStatus {
    pub status: String
}
