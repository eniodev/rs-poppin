use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct ReqInfo {
    message: String
}

#[get("/pop/{message}")]
async fn endpoint(info: web::Path<ReqInfo>) -> impl Responder {
    HttpResponse::Ok().body(info.message.to_owned())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(endpoint)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}