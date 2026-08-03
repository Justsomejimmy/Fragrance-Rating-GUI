use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Fragrance {
    pub id: i32,
    pub brand: String,
    pub name: String,
    pub concentration: String,
    pub rating: f32,
    pub notes: String,
    pub seasons: String,
}