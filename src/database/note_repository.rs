use rusqlite::Connection;

pub fn get_all(conn: &Connection) -> Vec<String> {
    let mut stmt = conn
        .prepare("SELECT name FROM note_options ORDER BY name COLLATE NOCASE")
        .unwrap();

    let rows = stmt.query_map([], |row| row.get::<_, String>(0)).unwrap();
    rows.filter_map(|r| r.ok()).collect()
}

pub fn insert_if_missing(conn: &Connection, name: &str) {
    conn.execute(
        "INSERT OR IGNORE INTO note_options (name) VALUES (?1)",
        [name],
    ).unwrap();
}