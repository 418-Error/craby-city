use crate::city::city_service::CityService;
use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Arc;

#[get("/cities")]
pub async fn get_all_cities(city_service: web::Data<Arc<CityService>>) -> impl Responder {
    match city_service.find_all().await {
        Ok(cities) => HttpResponse::Ok().json(cities),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
