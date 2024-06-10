use actix_web::{web, App, HttpServer, Responder, HttpResponse};

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Smart Warehouse Management System!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
