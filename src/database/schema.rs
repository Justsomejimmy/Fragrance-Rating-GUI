use rusqlite::Connection;

pub fn create_tables(conn: &Connection) {
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS fragrances (
            id INTEGER PRIMARY KEY,
            brand TEXT NOT NULL,
            name TEXT NOT NULL,
            concentration TEXT NOT NULL,
            rating REAL NOT NULL,
            notes TEXT,
            seasons TEXT
        )
        ",
        [],
    ).unwrap();
}

pub fn seed_database(conn: &Connection) {
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM fragrances",
            [],
            |row| row.get(0),
        ).unwrap();

    if count > 0 {
        return;
    }

    conn.execute(
        "
        INSERT INTO fragrances
        (brand, name, concentration, rating, notes, seasons)
        VALUES
        (
            'Chanel',
            'Bleu de Chanel',
            'EDP',
            9.2,
            'Grapefruit,Cedar,Incense',
            'Summer,Fall'
        )
        ",
        [],
    ).unwrap();
}