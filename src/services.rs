use cached::{proc_macro::cached, TimedCache};
use meilisearch_sdk::{
    client::Client, 
    indexes::Index, 
    search::SearchResult
};

use tokio::{ time, time::Duration };

use crate::models::{ Hentai, MeilisearchStatus };

pub async fn create_client() -> Index {
    let client = Client::new("http://0.0.0.0:7700", "masterKey");
    
    client.index("hentai")
}

pub async fn ping() {
    let delay = Duration::from_millis(100);

    loop {
        time::sleep(delay).await;

        match reqwest::get("http://localhost:7700/health").await {
            Ok(response) => match response.text().await {
                Ok(body) => match serde_json::from_str::<MeilisearchStatus>(&body) {
                    Ok(status) => {
                        if status.status == "available" {
                            break
                        }
                    },
                    Err(_) => {}
                },
                Err(_) => {}
            },
            Err(_) => {}
        }
    }
}

#[cached(
    type = "TimedCache<String, Vec<u32>>",
    create = "{ TimedCache::with_lifespan(6 * 3600) }",
    convert = r#"{ format!("{}{}",keyword, batch) }"#
)]
pub async fn search<'a>(engine: &Index, keyword: String, batch: usize) -> Vec<u32> {
    match engine.search()        
        .with_query(&keyword)
        .with_limit(25)
        .with_offset((batch - 1) * 25)
        .execute()
        .await {
        Ok(results) => results
            .hits
            .into_iter()
            .map(|hit: SearchResult<Hentai>| hit.result.id)
            .collect(),
        Err(a) => {
            println!("{}", a);
            
            vec![]
        },
    }
}
