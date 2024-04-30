use crate::city::city_service::CityService;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;

mod city;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    println!("Server started at http://localhost:3333");

    let pool = Arc::new(pool);

    let city_service = Arc::new(CityService::new(pool));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(city_service.clone()))
            .service(city::city_controller::get_all_cities)
    })
    .bind("127.0.0.1:3333")?
    .run()
    .await
}
