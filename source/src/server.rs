mod lib;

use actix_web::http::{ContentEncoding, StatusCode};
use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer};
use lib::custom_database::DiscordDatabase;
use std::env;
use std::sync::Mutex;

struct AppState {
    bd: Mutex<DiscordDatabase>,
}

async fn resource_query(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let query = req.match_info().get("query").unwrap_or("");
    let page = req.match_info().get("page").unwrap_or("0");
    let page = page.parse::<u16>().unwrap();

    let resources = &data.bd.lock().unwrap().select_resources(query, 10, page);

    if resources.len() == 0 {
        return HttpResponse::build(StatusCode::NOT_FOUND)
            .set_header(http::header::CONTENT_TYPE, "text/json")
            .set_header(
                http::header::CONTENT_ENCODING,
                ContentEncoding::Identity.as_str(),
            )
            .finish();
    }
    return HttpResponse::build(StatusCode::OK)
        .set_header(http::header::CONTENT_TYPE, "text/json")
        .set_header(
            http::header::CONTENT_ENCODING,
            ContentEncoding::Identity.as_str(),
        )
        .json(&resources);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string()); //.expect("Host not set");
    let host = host.as_str();
    let port = env::var("PORT").unwrap_or("8085".to_string()); //;
    let port = port.parse::<u16>().unwrap();

    dotenv::dotenv().expect("Failed to load .env file");
    let database_uri = env::var("DATABASE_URI").expect("Expected a token in the environment");

    let state = web::Data::new(AppState {
        // Setting database
        bd: Mutex::new(DiscordDatabase::new(database_uri)),
    });

    println!("Server start at http://{}:{}", host, port);
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(web::resource("/{query}/{page}").to(resource_query))
            .service(web::resource("/{query}").to(resource_query))
    })
    .bind((host, port))?
    .run()
    .await
}
