use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Fragrance {
    pub id: i32,
    pub brand: String,
    pub name: String,
    pub concentration: String,
    pub projection: String,
    pub longevity: String,
    pub price: String,
    pub purchase_date: String,
    pub rating: f32,
    pub notes: String,
    pub seasons: String,
    pub image_path: String,
    pub my_notes: String,
    pub partner_notes: String,
}

pub struct NewFragrance<'a> {
    pub brand: &'a str,
    pub name: &'a str,
    pub rating: f64,
    pub concentration: &'a str,
    pub projection: &'a str,
    pub longevity: &'a str,
    pub price: &'a str,
    pub purchase_date: &'a str,
    pub notes: &'a str,
    pub seasons: &'a str,
    pub image_path: &'a str,
    pub my_notes: &'a str,
    pub partner_notes: &'a str,
}