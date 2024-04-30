use std::sync::Arc;
use sqlx::{PgPool};
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
}