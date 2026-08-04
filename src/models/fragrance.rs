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