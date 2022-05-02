mod models;
mod controllers;
mod services;

use std::time::Duration;
use actix_web::{App, HttpServer, web::Data };
use controllers::{ sirin, sirin_batch };
use services::create_client;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    tokio::time::sleep(Duration::from_millis(100)).await;

    let client = create_client().await;
    let state = Data::new(client.index("hentai"));

    println!("Sirin Ready");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(sirin)
            .service(sirin_batch)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
