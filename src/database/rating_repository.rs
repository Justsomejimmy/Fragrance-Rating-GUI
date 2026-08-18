use rusqlite::Connection;

pub struct RatingRow {
    pub fragrance_id: i64,
    pub user_id: i64,
    pub rating: f64,
}

pub fn get_all(conn: &Connection) -> Vec<RatingRow> {
    let mut stmt = conn.prepare("SELECT fragrance_id, user_id, rating FROM ratings").unwrap();

    let rows = stmt.query_map([], |row| {
        Ok(RatingRow {
            fragrance_id: row.get(0)?,
            user_id: row.get(1)?,
            rating: row.get(2)?,
        })
    }).unwrap();

    rows.map(|r| r.unwrap()).collect()
}

pub fn set_rating(conn: &Connection, fragrance_id: i64, user_id: i64, rating: f64) {
    conn.execute(
        "
        INSERT INTO ratings (fragrance_id, user_id, rating)
        VALUES (?1, ?2, ?3)
        ON CONFLICT(fragrance_id, user_id) DO UPDATE SET rating = excluded.rating
        ",
        rusqlite::params![fragrance_id, user_id, rating],
    ).unwrap();
}

pub fn delete_for_fragrance(conn: &Connection, fragrance_id: i64) {
    conn.execute("DELETE FROM ratings WHERE fragrance_id = ?1", [fragrance_id]).unwrap();
}