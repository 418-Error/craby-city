use std::sync::Arc;

use sqlx::PgPool;

use crate::city::model::{City, CreateCity};

#[derive(Debug)]
enum CityError {
    CityNotFound,
}

#[allow(dead_code)]
async fn retrieve_city_service(pool: Arc<PgPool>, city: CreateCity) -> Result<City, CityError> {
    match sqlx::query_as::<_, City>("SELECT * FROM public.cities WHERE name = $1 and department_code = $2 and insee_code = $3 and zip_code = $4 and lat = $5 and lon = $6")
        .bind(city.name)
        .bind(city.department_code)
        .bind(city.insee_code)
        .bind(city.zip_code)
        .bind(city.lat)
        .bind(city.lon)
        .fetch_one(pool.as_ref())
        .await {
            Ok(city) => Ok(city),
            Err(_) => {
                Err(CityError::CityNotFound)
            },
        }
}

#[allow(dead_code)]
async fn clean_up_insertion(pool: Arc<PgPool>, city_id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM public.cities WHERE id = $1")
        .bind(city_id)
        .execute(pool.as_ref())
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::city::{city_service::CityService, model::CreateCity};
    use dotenv::dotenv;

    use super::*;

    #[sqlx::test]
    async fn test_find_all(pool: PgPool) -> Result<(), sqlx::Error> {
        dotenv().ok();
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");
        let pool = Arc::new(pool);
        let city_service = CityService::new(pool);
        let cities = city_service.find_all().await.unwrap();
        assert_ne!(
            cities.len(),
            0,
            "There should be cities in the database, currently there are {} cities",
            cities.len()
        );
        Ok(())
    }

    #[sqlx::test(fixtures(path = "./fixtures/clean.sql"))]
    async fn test_create(pool: PgPool) {
        dotenv().ok();
        let create_payload = CreateCity {
            department_code: "01".to_string(),
            insee_code: "01001".to_string(),
            zip_code: "01000".to_string(),
            name: "Bourg-en-Bresse".to_string(),
            lat: 46.205167,
            lon: 5.225723,
        };
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");
        let arc_pool = Arc::new(pool);
        let city_service = CityService::new(arc_pool.clone());
        city_service
            .create(create_payload.clone())
            .await
            .expect("Failed to create city");

        let test_retrieve = retrieve_city_service(arc_pool.clone(), create_payload).await;

        let city = test_retrieve.unwrap();

        assert_eq!(
            city.name, "Bourg-en-Bresse",
            "City name should be {}, but it is {}",
            city.name, "Bourg-en-Bresse"
        );
    }
}
