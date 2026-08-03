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
        rating,
        notes,
        seasons
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
                    rating: row.get(4)?,
                    notes: row.get(5)?,
                    seasons: row.get(6)?,
                }
            )
        }
    ).unwrap();

    fragrances.map(|f| f.unwrap()).collect()
}