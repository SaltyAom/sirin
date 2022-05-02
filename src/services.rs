use std::{borrow::BorrowMut, fs::File, io::prelude::Read, sync::Arc};

use cached::{proc_macro::cached, TimedCache};
use futures::future::join_all;
use meilisearch_sdk::{client::*, indexes::*, search::*};
use tokio::sync::Mutex;

use crate::models::Hentai;

pub async fn create_client() -> Client {
    let meilisearch = Client::new("http://0.0.0.0:7700", "masterKey");

    let shared_client = Arc::new(Mutex::new(meilisearch));
    let mut handler = vec![];

    // 1 to 20
    for iteration in 1..21 {
        let client = shared_client.clone();

        handler.push(tokio::spawn(async move {
            import_search(client, iteration)
                .await
                .expect("Unable to initialize Client");
        }));
    }

    join_all(handler).await;

    let client = shared_client.as_ref().lock().await.to_owned();

    client
}

pub async fn import_search(client: Arc<Mutex<Client>>, batch: u8) -> Result<(), std::io::Error> {
    let mut file = File::open(format!("data/searchable{}.json", batch))?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;
    let document: Vec<Hentai> = serde_json::from_str(&content)?;

    let engine = client.lock().await.borrow_mut().index("hentai");

    engine
        .add_documents(&document, Some("id"))
        .await
        .expect("Unable to connect to MeiliSearch");

    engine
        .set_sortable_attributes(&["id"])
        .await
        .expect("Unable to set sortable attributes");

    Ok(())
}

#[cached(
    type = "TimedCache<String, Vec<u32>>",
    create = "{ TimedCache::with_lifespan(6 * 3600) }",
    convert = r#"{ format!("{}{}",keyword, batch) }"#
)]
pub async fn search<'a>(engine: &Index, keyword: String, batch: usize) -> Vec<u32> {
    let query = Query::new(engine)
        .with_query(&keyword)
        .with_limit(25)
        .with_offset(batch * 25)
        .with_sort(&["id:desc"])
        .build();

    match engine.execute_query(&query).await {
        Ok(results) => results
            .hits
            .into_iter()
            .map(|hit: SearchResult<Hentai>| hit.result.id)
            .collect(),
        Err(_) => vec![],
    }
}
