mod city;
mod health;
mod tests;

use crate::city::city_service::CityService;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use actix_web_prometheus::PrometheusMetricsBuilder;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tracing::info;

use crate::city::city_controller;
use crate::health::health_controller;
use crate::health::health_service::HealthService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let addr = env::var("CITY_API_ADDR").unwrap_or("0.0.0.0".to_string());
    let port = env::var("CITY_API_PORT").unwrap_or("2022".to_string());
    let db_password =
        env::var("CITY_API_DB_PASSWORD").expect("CITY_API_DB_PASSWORD must be set in .env file");
    let db_url = env::var("CITY_API_DB_URL").expect("CITY_API_DB_URL must be set in .env file");
    let db_user = env::var("CITY_API_DB_USER").expect("CITY_API_DB_USER must be set in .env file");

    let database_url = format!(
        "postgres://{}:{}@{}/craby_city",
        db_user, db_password, db_url
    );

    let addr_in = format!("{}:{}", addr, port);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    info!("Starting server at: {}", addr_in);

    let pool = Arc::new(pool);

    let city_service = Arc::new(CityService::new(pool.clone()));
    let health_service = Arc::new(HealthService::new(pool.clone()));

    let mut labels = HashMap::new();
    labels.insert("label1".to_string(), "value1".to_string());
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .const_labels(labels)
        .build()
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone())
            .app_data(Data::new(city_service.clone()))
            .app_data(Data::new(health_service.clone()))
            .service(city_controller::get_all_cities)
            .service(city_controller::create_city)
            .service(health_controller::live)
            .service(health_controller::ready)
    })
    .bind(addr_in)?
    .run()
    .await
}
