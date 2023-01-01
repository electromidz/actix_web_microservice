use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod articles;
use articles::services;

struct AppState {
    articles: Mutex<Vec<Articles>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Articles {
    id: i32,
    author: String,
    title: String,
    content: String,
    lg_content: String,
    date: u64,
}

#[get("/")]
async fn index() -> String {
    "This is a health check route".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        articles: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
