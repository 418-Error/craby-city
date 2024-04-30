use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct City {
    pub id: u32,
    pub department_code: String,
    pub insee_code: String,
    pub zip_code: String,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}