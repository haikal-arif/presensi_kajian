use actix_files as fs;
use actix_web::{self, get, error, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Error};
use dotenv::dotenv;
use tera::{self, Tera};

use std::io;

async fn index(
    tmpl: web::Data<Tera>,
    _req: HttpRequest,
) -> Result<impl Responder, actix_web::Error> {
    let list_santri = vec!["Abdul", "Fuad", "Haikal"];
    let mut ctx = tera::Context::new();
    ctx.insert("list_santri", &list_santri);
    let rendered_page = tmpl
        .render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().body(rendered_page))
}


#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let addr = std::env::var("ADDRESS").expect("ADDRESS must be set.");

    println!("Listening on: {}, open browser and visit have a try!", addr);
    HttpServer::new(move || {
        let tera = match Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/views/templates/**/*")) {
            Ok(t) => t,
            Err(e) => {
                print!("Parsing error(s): {} ", e);
                std::process::exit(1);
            }
        };
        App::new()
            .app_data(web::Data::new(tera))
            .wrap(actix_web::middleware::Logger::default())
            .route("/", web::get().to(index))
            .service(fs::Files::new("/", "./views/static").show_files_listing())
    })
    .bind(addr)
    .expect("Address and port must not used")
    .run()
    .await
}
