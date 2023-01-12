use actix_files as fs;
use actix_web::{web, Error as AWError, HttpRequest, HttpResponse, Responder};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tera;

use super::db;

async fn index(
    tmpl: web::Data<tera::Tera>,
    dbpool: web::Data<db::SqlitePool>,
    _req: HttpRequest,
) -> Result<impl Responder, AWError> {
    let list_santri = db::get_nama_santri(dbpool).await.unwrap();
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
    alasan: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FormResponse {
    msg: String,
}

async fn submit(
    item: web::Json<FormSubmission>,
    dbpool: web::Data<db::SqlitePool>,
) -> Result<web::Json<FormResponse>, AWError> {
    let kehadiran = &item.status_hadir;

    let status = match kehadiran.as_str() {
        "hadir" => "hadir",
        "absen" => &item.alasan,
        _ => "This should not print",
    };

    db::submit_presensi(&item.nama, item.tanggal, status, dbpool)
        .await
        .expect("Error: gagal submit presensi !");
    // .map_err(|_| {
    //     actix_web::error::ErrorBadRequest("Error: Tanggal tidak valid !")
    // })?;

    Ok(web::Json(FormResponse {
        msg: format!(
            "Presensi sudah tercatat. Jazaakallahu Khairan {}",
            item.nama
        ),
    }))
}

pub fn config_app(
    tera_instance: tera::Tera,
    dbpool: Pool<SqliteConnectionManager>,
) -> Box<dyn Fn(&mut web::ServiceConfig)> {
    Box::new(move |cfg: &mut web::ServiceConfig| {
        cfg.app_data(web::Data::new(dbpool.clone()))
            .app_data(web::Data::new(tera_instance.clone()))
            .route("/", web::get().to(index))
            .route("/submitPresensi", web::post().to(submit))
            .service(fs::Files::new("/", "views/static").show_files_listing());
    })
}
