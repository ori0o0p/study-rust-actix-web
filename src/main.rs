mod service;
mod model;

use actix_cors::Cors;
use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};
use crate::model::UploadArticleDto;

#[get("/api/{id}")]
async fn get_article(id: web::Path<u32>) -> HttpResponse {
    HttpResponse::Ok()
        .json(service::get_article_default(id.clone()).await)
}

#[get("/api")]
async fn find_all() -> HttpResponse {
    HttpResponse::Ok()
        .json(service::find_all().await)
}

#[post("/api")]
async fn upload_article(upload_article_dto: web::Json<UploadArticleDto>) -> HttpResponse {
    service::upload_article(upload_article_dto.into_inner()).await;

    HttpResponse::Created()
        .body(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let num_workers = num_cpus::get();

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default())
            .service(get_article)
            .service(upload_article)
            .service(find_all)
    })
        .workers(num_workers)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
