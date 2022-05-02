mod controllers;
mod models;
mod services;

use actix_web::{web::Data, App, HttpServer};
use controllers::{hi, sirin, sirin_batch};
use services::create_client;
use std::time::Duration;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Request Sirin");

    tokio::time::sleep(Duration::from_millis(200)).await;

    let client = create_client().await;
    let state = Data::new(client.index("hentai"));

    println!("Sirin Ready");

    HttpServer::new(move || {
        App::new().app_data(state.clone()).service(hi)
        // .service(sirin)
        // .service(sirin_batch)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
