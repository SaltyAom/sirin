use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Hentai {
    pub id: u32,
    title: String,
    tags: Vec<String>,
    page: u16,
}