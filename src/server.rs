use actix_web::{web, HttpRequest, HttpResponse, Responder, Error as AWError};
use actix_files as fs;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tera;

use super::db;


async fn index(
    tmpl: web::Data<tera::Tera>,
    db: web::Data<db::SqlitePool>,
    _req: HttpRequest,
) -> Result<impl Responder, AWError> {
    let list_santri = db::get_nama_santri(db).await.unwrap();
    let mut ctx = tera::Context::new();
    ctx.insert("list_santri", &list_santri);
    let rendered_page = tmpl
        .render("index.html", &ctx)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().body(rendered_page))
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
    println!("Tanggal {}", item.tanggal);
    println!("Kehadiran : {}", item.status_hadir);
    Ok(format!("Accepted {}", item.nama))
}


pub fn config_app(tera_instance: tera::Tera, dbpool: Pool<SqliteConnectionManager>) -> Box<dyn Fn(&mut web::ServiceConfig)> {
    Box::new(move |cfg: &mut web::ServiceConfig| {
        cfg.app_data(web::Data::new(dbpool.clone()))
            .app_data(web::Data::new(tera_instance.clone()))
            .route("/", web::get().to(index))
            .route("/submitPresensi", web::post().to(submit))
            .service(fs::Files::new("/", "views/static").show_files_listing());
    })    
}