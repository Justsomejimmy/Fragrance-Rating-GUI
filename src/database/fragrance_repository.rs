use rusqlite::Connection;
use crate::models::fragrance::{Fragrance, NewFragrance, UpdateFragrance};

pub fn get_all(conn: &Connection) -> Vec<Fragrance> {
    let mut stmt = conn.prepare(
        "
        SELECT
            id, brand, name, concentration, projection, longevity,
            price, purchase_date, rating, notes, seasons, image_path,
            my_notes, partner_notes, image_offset_x, image_offset_y,
            image_scale, category
        FROM fragrances
        "
    ).unwrap();

    let fragrances = stmt.query_map([], |row| {
        Ok(Fragrance {
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
            image_offset_x: row.get(14)?,
            image_offset_y: row.get(15)?,
            image_scale: row.get(16)?,
            category: row.get(17)?,
        })
    }).unwrap();

    fragrances.map(|f| f.unwrap()).collect()
}

pub fn insert(conn: &Connection, fragrance: NewFragrance) {
    conn.execute(
        "
        INSERT INTO fragrances (
            brand, name, concentration, projection, longevity, price,
            purchase_date, rating, notes, seasons, image_path, my_notes,
            partner_notes, image_offset_x, image_offset_y, image_scale, category
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)
        ",
        rusqlite::params![
            fragrance.brand,
            fragrance.name,
            fragrance.concentration,
            fragrance.projection,
            fragrance.longevity,
            fragrance.price,
            fragrance.purchase_date,
            fragrance.rating,
            fragrance.notes,
            fragrance.seasons,
            fragrance.image_path,
            fragrance.my_notes,
            fragrance.partner_notes,
            fragrance.image_offset_x,
            fragrance.image_offset_y,
            fragrance.image_scale,
            fragrance.category,
        ],
    ).unwrap();
}

pub fn update(conn: &Connection, fragrance: UpdateFragrance) {
    conn.execute(
        "
        UPDATE fragrances
        SET
            brand = ?1, name = ?2, rating = ?3, concentration = ?4,
            projection = ?5, longevity = ?6, price = ?7, purchase_date = ?8,
            notes = ?9, seasons = ?10, my_notes = ?11, partner_notes = ?12,
            image_path = ?13, image_offset_x = ?14, image_offset_y = ?15,
            image_scale = ?16, category = ?17
        WHERE id = ?18
        ",
        rusqlite::params![
            fragrance.brand,
            fragrance.name,
            fragrance.rating,
            fragrance.concentration,
            fragrance.projection,
            fragrance.longevity,
            fragrance.price,
            fragrance.purchase_date,
            fragrance.notes,
            fragrance.seasons,
            fragrance.my_notes,
            fragrance.partner_notes,
            fragrance.image_path,
            fragrance.image_offset_x,
            fragrance.image_offset_y,
            fragrance.image_scale,
            fragrance.category,
            fragrance.id,
        ],
    ).unwrap();
}

pub fn delete(conn: &Connection, id: i32) {
    conn.execute("DELETE FROM fragrances WHERE id = ?1", [id]).unwrap();
}