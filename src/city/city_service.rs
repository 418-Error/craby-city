use crate::city::model::{City, CreateCity};
use sqlx::PgPool;
use std::sync::Arc;

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

    pub async fn create(&self, city: CreateCity) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO public.cities (department_code, insee_code, zip_code, name, lat, lon) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(city.department_code)
            .bind(city.insee_code)
            .bind(city.zip_code)
            .bind(city.name)
            .bind(city.lat)
            .bind(city.lon)
            .execute(self.pool.as_ref())
            .await?;

        Ok(())
    }
}
