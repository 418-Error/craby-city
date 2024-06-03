use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct City {
    pub id: i32,
    pub department_code: String,
    pub insee_code: Option<String>,
    pub zip_code: Option<String>,
    pub name: String,
    pub lat: f32,
    pub lon: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCity {
    pub department_code: String,
    pub insee_code: String,
    pub zip_code: String,
    pub name: String,
    pub lat: f32,
    pub lon: f32,
}
