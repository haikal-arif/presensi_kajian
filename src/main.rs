use actix_web::{self, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use tera::{self, Tera};

mod db;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env setup
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let addr = std::env::var("ADDRESS").expect("ADDRESS must be set to a valid port and ip.");
    let db_file = std::env::var("DBFILE").expect("DBFILE must be set to a valid db file");

    // DB Setup
    let manager = r2d2_sqlite::SqliteConnectionManager::file(db_file.clone());
    let pool =
        r2d2::Pool::new(manager).expect(format!("Database should be at {}", db_file).as_str());

    // Template Renderer
    let tera_instance = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/views/templates/**/*"))
        .expect("Template folder should be here");

    println!("⚙️  Server listening to {}", addr);
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .configure(server::config_app(tera_instance.clone(), pool.clone()))
    })
    .bind(addr)
    .expect("Address and port must not used")
    .run()
    .await
}
