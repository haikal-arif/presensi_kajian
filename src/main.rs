use actix_files as fs;
use actix_web::{
    self, error, web, App, Error as AWError, HttpRequest, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use tera::{self, Tera};
use chrono;

type SqlitePool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

async fn index(
    tmpl: web::Data<Tera>,
    db: web::Data<SqlitePool>,
    _req: HttpRequest,
) -> Result<impl Responder, AWError> {
    let list_santri = get_nama_santri(db).await.unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("list_santri", &list_santri);
    let rendered_page = tmpl
        .render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().body(rendered_page))
}

struct Santri {
    nama: String,
    
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FormSubmission {
    nama: String,
    tanggal: chrono::NaiveDate,
    status_hadir: String,
    alasan: String
}

async fn submit(item: web::Json<FormSubmission>) -> Result<String, AWError> {
    println!("Accepted {}", item.nama);
    Ok(format!("Accepted {}", item.nama))
}

async fn get_nama_santri(pool: web::Data<SqlitePool>) -> Result<Vec<String>, AWError> {
    let pool = pool.clone();
    let conn = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;

    let result: Result<Vec<Santri>, rusqlite::Error> = conn
        .prepare("SELECT nama FROM presensi")
        .map_err(error::ErrorInternalServerError)?
        .query_map([], |row| Ok(Santri { nama: row.get(0)? }))
        .and_then(Iterator::collect);

    let nama_santri: Vec<String> = match result {
        Ok(res) => res.iter().map(|santri| santri.nama.clone()).collect(),
        Err(_) => {
            vec!["Failed to Fetch".to_string()]
        }
    };

    Ok(nama_santri)
}

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
            .app_data(web::Data::new(pool.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/submitPresensi", web::post().to(submit))
            .service(fs::Files::new("/", "./views/static").show_files_listing())
    })
    .bind(addr)
    .expect("Address and port must not used")
    .run()
    .await
}
