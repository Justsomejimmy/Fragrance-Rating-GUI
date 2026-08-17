use rusqlite::Connection;

pub fn create_tables(conn: &Connection) {
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS fragrances (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            brand TEXT NOT NULL,
            name TEXT NOT NULL,
            concentration TEXT,
            projection TEXT,
            longevity TEXT,
            price TEXT,
            purchase_date TEXT,
            rating REAL,
            notes TEXT,
            seasons TEXT,
            image_path TEXT,
            my_notes TEXT,
            partner_notes TEXT,
            image_offset_x REAL NOT NULL DEFAULT 0,
            image_offset_y REAL NOT NULL DEFAULT 0,
            image_scale REAL NOT NULL DEFAULT 1.0
        )
        ",
        [],
    ).unwrap();

    // Migration for existing databases created before these columns existed.
    // ALTER TABLE ... ADD COLUMN errors if the column is already present,
    // so we just ignore that specific failure and continue.
    let _ = conn.execute(
        "ALTER TABLE fragrances ADD COLUMN image_offset_x REAL NOT NULL DEFAULT 0",
        [],
    );
    let _ = conn.execute(
        "ALTER TABLE fragrances ADD COLUMN image_offset_y REAL NOT NULL DEFAULT 0",
        [],
    );
    let _ = conn.execute(
        "ALTER TABLE fragrances ADD COLUMN image_scale REAL NOT NULL DEFAULT 1.0",
        [],
    );
}

pub fn seed_database(conn: &Connection) {
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM fragrances", [], |row| row.get(0))
        .unwrap();

    if count > 0 {
        return;
    }

    conn.execute(
        "
        INSERT INTO fragrances (
            brand, name, concentration, projection, longevity, price,
            purchase_date, rating, notes, seasons, image_path, my_notes,
            partner_notes, image_offset_x, image_offset_y, image_scale
        )
        VALUES
        (
            'Chanel', 'Bleu de Chanel', 'EDP', 'Moderate', 'Long', '$120',
            '2025-08-01', 9.2, 'Grapefruit,Cedar,Incense', 'Summer,Fall',
            '', 'Excellent evening fragrance.', 'Fresh and classy.', 0, 0, 1.0
        )
        ",
        [],
    ).unwrap();
}