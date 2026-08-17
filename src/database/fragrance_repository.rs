use rusqlite::Connection;
use crate::models::fragrance::Fragrance;

pub fn get_all(
    conn: &Connection
) -> Vec<Fragrance> {

    let mut stmt = conn.prepare(
        "
        SELECT
            id,
            brand,
            name,
            concentration,
            projection,
            longevity,
            price,
            purchase_date,
            rating,
            notes,
            seasons,
            image_path,
            my_notes,
            partner_notes
        FROM fragrances
        "
    ).unwrap();

    let fragrances = stmt.query_map(
        [],
        |row| {
            Ok(
                Fragrance {
                    id: row.get(0)?,
                    brand: row.get(1)?,
                    name: row.get(2)?,
                    concentration: row.get(3)?,
                    projection: row.get(4)?,
                    longevity: row.get(5)?,
                    price: row.get(6)?,
                    purchase_date: row.get(7)?,
                    rating: row.get(8)?,
                    notes: row.get(9)?,
                    seasons: row.get(10)?,
                    image_path: row.get(11)?,
                    my_notes: row.get(12)?,
                    partner_notes: row.get(13)?,
                }
            )
        }
    ).unwrap();

    fragrances.map(|f| f.unwrap()).collect()
}

pub fn update(
    conn: &Connection,
    id: i64,
    brand: &str,
    name: &str,
    rating: f64,
    concentration: &str,
    projection: &str,
    longevity: &str,
    price: &str,
    purchase_date: &str,
    notes: &str,
    seasons: &str,
    my_notes: &str,
    partner_notes: &str,
) {
    conn.execute(
        "
        UPDATE fragrances
        SET
            brand = ?1,
            name = ?2,
            rating = ?3,
            concentration = ?4,
            projection = ?5,
            longevity = ?6,
            price = ?7,
            purchase_date = ?8,
            notes = ?9,
            seasons = ?10,
            my_notes = ?11,
            partner_notes = ?12
        WHERE id = ?13
        ",
        (
            brand,
            name,
            rating,
            concentration,
            projection,
            longevity,
            price,
            purchase_date,
            notes,
            seasons,
            my_notes,
            partner_notes,
            id,
        ),
    )
    .unwrap();
}

pub fn delete(conn: &Connection, id: i32) {
    conn.execute(
        "DELETE FROM fragrances WHERE id = ?1",
        [id],
    )
    .unwrap();
}

pub fn insert(
    conn: &rusqlite::Connection,
    brand: &str,
    name: &str,
    rating: f64,
    concentration: &str,
    projection: &str,
    longevity: &str,
    price: &str,
    purchase_date: &str,
    notes: &str,
    seasons: &str,
    image_path: &str,
    my_notes: &str,
    partner_notes: &str,
) {
    conn.execute(
        "
        INSERT INTO fragrances (
            brand,
            name,
            concentration,
            projection,
            longevity,
            price,
            purchase_date,
            rating,
            notes,
            seasons,
            image_path,
            my_notes,
            partner_notes
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
        ",
        rusqlite::params![
            brand,
            name,
            concentration,
            projection,
            longevity,
            price,
            purchase_date,
            rating,
            notes,
            seasons,
            image_path,
            my_notes,
            partner_notes
        ],
    )
    .unwrap();
}