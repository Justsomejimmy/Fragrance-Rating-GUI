use rusqlite::Connection;
use crate::models::user::User;

pub fn get_all(conn: &Connection) -> Vec<User> {
    let mut stmt = conn.prepare("SELECT id, name FROM users ORDER BY id").unwrap();

    let rows = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    }).unwrap();

    rows.map(|r| r.unwrap()).collect()
}

pub fn insert(conn: &Connection, name: &str) {
    conn.execute("INSERT INTO users (name) VALUES (?1)", [name]).unwrap();
}

pub fn update(conn: &Connection, id: i64, name: &str) {
    conn.execute("UPDATE users SET name = ?1 WHERE id = ?2", rusqlite::params![name, id]).unwrap();
}

pub fn delete(conn: &Connection, id: i64) {
    conn.execute("DELETE FROM ratings WHERE user_id = ?1", [id]).unwrap();
    conn.execute("DELETE FROM users WHERE id = ?1", [id]).unwrap();
}