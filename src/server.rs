use actix_files as fs;
use actix_web::{web, Error as AWError, HttpRequest, HttpResponse, Responder};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tera::{self, Tera};

use super::db;

pub fn config_app(
    tera_instance: Tera,
    dbpool: Pool<SqliteConnectionManager>,
) -> Box<dyn Fn(&mut web::ServiceConfig)> {
    Box::new(move |cfg: &mut web::ServiceConfig| {
        cfg.app_data(web::Data::new(dbpool.clone()))
            .app_data(web::Data::new(tera_instance.clone()))
            .service(index)
            .service(submit)
            .service(register)
            .service(fs::Files::new("/", "views/static").show_files_listing());
    })
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

#[derive(serde::Serialize, serde::Deserialize)]
struct RegistrationForm {
    nama: String,
}

#[actix_web::get("/")]
async fn index(
    tmpl: web::Data<Tera>,
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

#[actix_web::post("/submitPresensi")]
async fn submit(
    item: web::Json<FormSubmission>,
    dbpool: web::Data<db::SqlitePool>,
) -> Result<web::Json<FormResponse>, AWError> {
    let kehadiran = &item.status_hadir;

    let status = match kehadiran.as_str() {
        "hadir" => Ok("hadir"),
        "absen" => Ok(item.alasan.as_str()),
        _ => Err(actix_web::error::ErrorBadRequest("Invalid Value")),
    }?;

    db::submit_presensi(&item.nama, item.tanggal, status, dbpool)
        .await
        .map_err(|err| {
            let err_msg = format!("Gagal submit presensi : {}", err);
            actix_web::error::ErrorInternalServerError(err_msg)
        })?;

    Ok(web::Json(FormResponse {
        msg: format!(
            "Presensi sudah tercatat. Jazaakallahu Khairan {}",
            item.nama
        ),
    }))
}

#[actix_web::post("/registerSantri")]
async fn register(
    item: web::Json<RegistrationForm>,
    dbpool: web::Data<db::SqlitePool>,
) -> Result<web::Json<FormResponse>, AWError> {
    let nama_santri = &item.nama;

    db::register_santri(nama_santri, dbpool)
        .await
        .map_err(|err| {
            let err_msg = format!("Pendaftaran gagal : {}", err);
            actix_web::error::ErrorInternalServerError(err_msg)
        })?;

    Ok(web::Json(FormResponse {
        msg: format!("Nama sudah terdaftar. Jazaakallahu Khairan {}", nama_santri),
    }))
}
