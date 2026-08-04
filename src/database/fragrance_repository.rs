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
    fragrance: &Fragrance
) {
    conn.execute(
        "
        UPDATE fragrances
        SET
            brand = ?,
            name = ?,
            concentration = ?,
            projection = ?,
            longevity = ?,
            price = ?,
            purchase_date = ?,
            rating = ?,
            notes = ?,
            seasons = ?,
            my_notes = ?,
            partner_notes = ?
        WHERE id = ?
        ",
        (
            &fragrance.brand,
            &fragrance.name,
            &fragrance.concentration,
            &fragrance.projection,
            &fragrance.longevity,
            &fragrance.price,
            &fragrance.purchase_date,
            fragrance.rating,
            &fragrance.notes,
            &fragrance.seasons,
            &fragrance.my_notes,
            &fragrance.partner_notes,
            fragrance.id,
        ),
    ).unwrap();
}