use std::sync::Arc;
use sqlx::{PgPool, Pool, Postgres};
use crate::city::model::City;

#[derive(Clone)]
pub struct CityService {
    pool: Arc<PgPool>,
}

impl CityService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<City>, sqlx::Error> {
        let cities = sqlx::query_as::<_, City>("SELECT * FROM public.cities;")
            .fetch_all(self.pool.as_ref())
            .await?;

        Ok(cities)
    }

    pub async fn find_by_id(&self, id: i32) -> City {
        let city = City {
            id: 1,
            name: "Paris".to_string(),
            zip_code: "75000".to_string(),
            insee_code: "75056".to_string(),
            department_code: "75".to_string(),
            lon: 2.3522,
            lat: 48.8566,
        };

        city
    }
}