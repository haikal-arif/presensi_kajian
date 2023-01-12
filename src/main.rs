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
    let addr = std::env::var("ADDRESS").expect("ADDRESS must be set.");

    // DB Setup
    let manager = r2d2_sqlite::SqliteConnectionManager::file("datasantri.db");
    let pool = r2d2::Pool::new(manager).unwrap();

    // Template Renderer
    let tera = match Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/views/templates/**/*")) {
        Ok(t) => t,
        Err(e) => {
            print!("Parsing error(s): {} ", e);
            std::process::exit(1);
        }
    };

    println!("Listening on: {}, open browser and visit have a try!", addr);
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .configure(server::config_app(tera.clone(), pool.clone()))
    })
    .bind(addr)
    .expect("Address and port must not used")
    .run()
    .await
}
