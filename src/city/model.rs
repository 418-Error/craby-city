use sqlx::FromRow;
use serde::Serialize;
use bigdecimal::BigDecimal;

#[derive(Debug, FromRow, Serialize)]
pub struct City {
    pub id: i32,
    pub department_code: String,
    pub insee_code: String,
    pub zip_code: String,
    pub name: String,
    pub lat: f32,
    pub lon: f32,
}