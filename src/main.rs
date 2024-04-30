use std::env;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions};
mod city;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    println!("Server started at http://localhost:3333");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
    })
        .bind("127.0.0.1:3333")?
        .run()
        .await
}
