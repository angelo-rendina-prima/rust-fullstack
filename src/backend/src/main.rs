mod todo;

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
            .route("/", actix_web::web::get().to(todo::get_all))
            .route("/", actix_web::web::post().to(todo::create))
            .route("/", actix_web::web::put().to(todo::resolve))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
