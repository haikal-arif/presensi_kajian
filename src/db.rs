use actix_web::{web, error, Error as AWError};

pub type SqlitePool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

struct Santri {
    nama: String,
}

pub async fn get_nama_santri(pool: web::Data<SqlitePool>) -> Result<Vec<String>, AWError> {
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
