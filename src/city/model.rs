use sqlx::FromRow;
use serde::Serialize;

#[derive(Debug, FromRow, Serialize)]
pub struct City {
    pub id: i64,
    pub department_code: String,
    pub insee_code: String,
    pub zip_code: String,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}