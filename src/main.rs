mod controllers;
mod models;
mod services;

use actix_web::{web::Data, App, HttpServer};
use controllers::{hi, sirin};
use services::{ create_client, ping };

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    ping().await;

    let state = Data::new(create_client().await);

    println!("Start Sirin");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(hi)
            .service(sirin)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
