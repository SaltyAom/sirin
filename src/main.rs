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

    let mut file = File::open(format!("data/searchable{}.json", 1))?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;
    let document: Vec<Hentai> = serde_json::from_str(&content)?;

    let engine = meilisearch.index("hentai");

    engine
        .add_documents_in_batches(&document, Some(30_000), None)
        .await
        .expect("Unable to connect to MeiliSearch");

    // engine
    //     .set_sortable_attributes(&["id"])
    //     .await
    //     .expect("Unable to set sortable attributes");

    // println!("Start Sirin");

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
