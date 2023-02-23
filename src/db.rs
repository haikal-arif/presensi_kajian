use actix_web::{error as aw_error, web, Error as AWError};
use rusqlite;

pub type SqlitePool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

struct Santri {
    nama: String,
}

pub async fn get_nama_santri(pool: web::Data<SqlitePool>) -> Result<Vec<String>, AWError> {
    let pool = pool.clone(); // web::Data<T> is cheap to clone
    let conn = web::block(move || pool.get())
        .await?
        .map_err(|err| aw_error::ErrorInternalServerError(err))?;

    let result: Result<Vec<Santri>, rusqlite::Error> = conn
        .prepare("SELECT nama FROM presensi")
        .map_err(|err| aw_error::ErrorInternalServerError(err))?
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

pub async fn submit_presensi(
    nama: &String,
    tanggal_presensi: chrono::NaiveDate,
    status: &str,
    dbpool: web::Data<SqlitePool>,
) -> Result<usize, AWError> {
    let basis_tanggal = chrono::NaiveDate::from_ymd_opt(2023, 1, 14).expect("This should not err");
    let selisih_hari = (tanggal_presensi - basis_tanggal).num_days();

    if selisih_hari < 0 {
        return Err(aw_error::ErrorBadRequest("Tanggal tidak valid"));
    }

    let batas_kolom: i64 = std::env::var("COLUMN_LIMIT")
        .expect("COLUMN_LIMIT must be set.")
        .parse::<i64>()
        .map_err(|err| aw_error::ErrorBadRequest(err))?;
    let mut column_num = selisih_hari / 7;

    if column_num > batas_kolom {
        return Err(aw_error::ErrorBadRequest("Tanggal di luar batas !"));
    }

    // map 0, 1, 7, 8, ...
    // into 0, 1, 2, 3, 4, ...
    column_num = match selisih_hari % 7 {
        0 => Ok(column_num * 2 + 0),
        1 => Ok(column_num * 2 + 1),
        _ => Err(aw_error::ErrorBadRequest("Bukan tanggal kajian !")),
    }?;

    let column_name = format!("column{}", column_num);

    let pool = dbpool.clone();
    let conn = web::block(move || pool.get()).await?.map_err(|err| {
        aw_error::ErrorInternalServerError(format!("Failed to connect to db: {}", err))
    })?;
    conn.execute(
        format!("UPDATE presensi SET {} = ?1 WHERE nama=?2", column_name).as_str(),
        (status, nama),
    )
    .map_err(|err| aw_error::ErrorInternalServerError(err))
}

pub async fn register_santri(
    nama: &String,
    dbpool: web::Data<SqlitePool>,
) -> Result<usize, AWError> {
    let pool = dbpool.clone();
    let conn = web::block(move || pool.get()).await?.map_err(|err| {
        aw_error::ErrorInternalServerError(format!("Failed to connect to db: {}", err))
    })?;
    conn.execute("INSERT INTO presensi (nama) VALUES (?1)", (nama,))
        .map_err(|err| aw_error::ErrorInternalServerError(err))
}
