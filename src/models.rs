use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Hentai {
    pub id: u32,
    title: String,
    tags: Vec<String>,
    page: u16,
}
