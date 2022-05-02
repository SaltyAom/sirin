mod controllers;
mod models;
mod services;

use actix_web::{web::Data, App, HttpServer};
use controllers::{hi, sirin, sirin_batch};
use meilisearch_sdk::client::Client;
// use services::create_client;
use std::{fs::File, io::Read, time::Duration};

use crate::models::Hentai;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Request Sirin");

    tokio::time::sleep(Duration::from_millis(1000)).await;

    // let client = create_client().await;
    // let state = Data::new(client.index("hentai"));
    let meilisearch = Client::new("http://0.0.0.0:7700", "masterKey");

    let mut file = File::open(format!("data/searchable{}.json", 3))?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;
    let document: Vec<Hentai> = serde_json::from_str(&content)?;

    let engine = match meilisearch.get_index("hentai").await {
        Ok(index) => index,
        Err(_) => {
            let index = meilisearch
                .create_index("hentai", Some("id"))
                .await
                .expect("Couldn't join the server")
                .wait_for_completion(&meilisearch, None, None)
                .await
                .expect("Couldn't join the remote server")
                .try_make_index(&meilisearch)
                .expect("Unable to create index");

            let task = index
                .set_sortable_attributes(&["id"])
                .await
                .expect("Unable to connect to MeiliSearch")
                .wait_for_completion(&meilisearch, None, None)
                .await
                .expect("Couldn't join the remote server");

            if task.is_failure() {
                panic!("Unable to apply settings")
            }

            index
        }
    };

    engine
        .add_or_update(&document, Some("id"))
        .await
        .expect("Unable to add documents to batch")
        .wait_for_completion(&meilisearch, None, None)
        .await
        .expect("Unable to join the remote server");

    println!("Start Sirin");

    HttpServer::new(move || {
        App::new()
            // .app_data(state.clone())
            .service(hi)
        // .service(sirin)
        // .service(sirin_batch)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
