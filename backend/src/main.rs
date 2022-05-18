#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new().route("/", actix_web::web::get().to(health_check))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().body("Ok")
}
