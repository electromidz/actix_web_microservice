use super::models::CreateArticle;
use crate::{AppState, Articles};
use actix_web::{get, post, put, web, HttpResponse, Responder};

#[get("/articles/entries")]
async fn get_entries(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.articles.lock().unwrap().to_vec())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_entries);
}
