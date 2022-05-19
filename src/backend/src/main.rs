mod auth;
mod user;

pub struct App {
    pool: sqlx::Pool<sqlx::Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let app = actix_web::web::Data::new(App {
        pool: sqlx::pool::PoolOptions::new()
            .connect("postgresql://postgres:postgres@localhost:5432/todo")
            .await
            .expect("Could not connect to the DB"),
    });

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(app.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_session::SessionMiddleware::new(
                actix_session::storage::CookieSessionStore::default(),
                actix_web::cookie::Key::generate(),
            ))
            .route("/", actix_web::web::get().to(actix_web::HttpResponse::Ok))
            .service(
                actix_web::web::scope("/auth")
                    .route("/whoami", actix_web::web::get().to(auth::whoami))
                    .route("/logout", actix_web::web::post().to(auth::logout))
                    .service(
                        actix_web::web::resource("/login/{email}/{password}")
                            .route(actix_web::web::post().to(auth::login)),
                    ),
            )
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
