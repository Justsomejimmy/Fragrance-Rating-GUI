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
            notes TEXT,
            seasons TEXT,
            image_path TEXT,
            my_notes TEXT,
            partner_notes TEXT,
            image_offset_x REAL NOT NULL DEFAULT 0,
            image_offset_y REAL NOT NULL DEFAULT 0,
            image_scale REAL NOT NULL DEFAULT 1.0,
            category TEXT NOT NULL DEFAULT 'Perfume',
            is_wishlist INTEGER NOT NULL DEFAULT 0
        )
        ",
        [],
    ).unwrap();

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        )
        ",
        [],
    ).unwrap();

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS ratings (
            fragrance_id INTEGER NOT NULL,
            user_id INTEGER NOT NULL,
            rating REAL NOT NULL,
            PRIMARY KEY (fragrance_id, user_id)
        )
        ",
        [],
    ).unwrap();

    // Migrations for pre-existing databases (safe to run repeatedly; errors ignored on purpose)
    let _ = conn.execute("ALTER TABLE fragrances ADD COLUMN image_offset_x REAL NOT NULL DEFAULT 0", []);
    let _ = conn.execute("ALTER TABLE fragrances ADD COLUMN image_offset_y REAL NOT NULL DEFAULT 0", []);
    let _ = conn.execute("ALTER TABLE fragrances ADD COLUMN image_scale REAL NOT NULL DEFAULT 1.0", []);
    let _ = conn.execute("ALTER TABLE fragrances ADD COLUMN category TEXT NOT NULL DEFAULT 'Perfume'", []);
    let _ = conn.execute("ALTER TABLE fragrances ADD COLUMN is_wishlist INTEGER NOT NULL DEFAULT 0", []);
    let _ = conn.execute("ALTER TABLE fragrances DROP COLUMN rating", []);
}

pub fn seed_database(conn: &Connection) {
    let fragrance_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM fragrances", [], |row| row.get(0))
        .unwrap();

    if fragrance_count == 0 {
        conn.execute(
            "
            INSERT INTO fragrances (
                brand, name, concentration, projection, longevity, price,
                purchase_date, notes, seasons, image_path, my_notes,
                partner_notes, image_offset_x, image_offset_y, image_scale, category, is_wishlist
            )
            VALUES
            (
                'Chanel', 'Bleu de Chanel', 'EDP', 'Moderate', 'Long', '$120',
                '2025-08-01', 'Grapefruit,Cedar,Incense', 'Summer,Fall',
                '', 'Excellent evening fragrance.', 'Fresh and classy.', 0, 0, 1.0, 'Cologne', 0
            )
            ",
            [],
        ).unwrap();
    }

    let user_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))
        .unwrap();

    if user_count == 0 {
        conn.execute("INSERT INTO users (name) VALUES ('Me')", []).unwrap();
        conn.execute("INSERT INTO users (name) VALUES ('Partner')", []).unwrap();
    }
}