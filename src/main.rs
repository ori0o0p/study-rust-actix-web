use actix_cors::Cors;
use actix_web::{App, get, HttpResponse, HttpServer, Responder};

#[get("/api")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body("아일릿")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let num_workers = num_cpus::get();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET"])
                    .allow_any_header(),
            )
            .service(hello)
    })
        .workers(num_workers)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
