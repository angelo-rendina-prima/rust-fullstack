mod auth;
mod todo;
mod user;

pub struct App {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl App {
    pub fn pool(&self) -> &sqlx::Pool<sqlx::Postgres> {
        &self.pool
    }
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
            .wrap(actix_cors::Cors::permissive())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_session::SessionMiddleware::builder(
                actix_session::storage::CookieSessionStore::default(),
                actix_web::cookie::Key::generate(),
            )
                .cookie_secure(false)
                .cookie_same_site(actix_web::cookie::SameSite::None)
                .cookie_http_only(false)
                .build()
        )
            .route("/", actix_web::web::get().to(actix_web::HttpResponse::Ok))
            .service(
                actix_web::web::scope("/auth")
                    .route("/", actix_web::web::get().to(auth::whoami))
                    .route("/logout", actix_web::web::post().to(auth::logout))
                    .route("/login", actix_web::web::post().to(auth::login)),
            )
            .service(
                actix_web::web::scope("/todo")
                    .route("/", actix_web::web::get().to(todo::get_all_for_active_user))
                    .route("/new", actix_web::web::post().to(todo::create))
                    .route("/resolve", actix_web::web::post().to(todo::resolve)),
            )
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
