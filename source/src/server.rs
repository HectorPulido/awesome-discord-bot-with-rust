mod lib;

use actix_files as fs;
use actix_web::{web, App, HttpServer};
use lib::app_state;
use lib::custom_database::DiscordDatabase;
use lib::endpoints::index::index;
use lib::endpoints::search::resource_query;
use std::env;
use std::sync::Mutex;
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to load .env file");

    let host = env::var("HOST").unwrap();
    let host = host.as_str();
    let port = env::var("PORT").unwrap();
    let port = port.parse::<u16>().unwrap();
    let database_uri = env::var("DATABASE_URI").expect("Expected a token in the environment");

    let state = web::Data::new(app_state::AppState {
        // Setting database
        bd: Mutex::new(DiscordDatabase::new(database_uri)),
    });

    println!("Server start at http://{}:{}", host, port);
    // println!("Cargo manifest dir: {}", env!("CARGO_MANIFEST_DIR"));
    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .app_data(state.clone())
            .data(tera)
            .service(fs::Files::new("/static", "./static"))
            .service(web::resource("/{query}/{page}").to(resource_query))
            .service(web::resource("/{query}").to(resource_query))
            .service(web::resource("/").to(index))
    })
    .bind((host, port))?
    .run()
    .await
}
