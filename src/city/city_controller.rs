use std::sync::Arc;
use actix_web::{get, HttpResponse, Responder, web};
use crate::city::city_service::CityService;

#[get("/cities")]
pub async fn get_all_cities(
    city_service: web::Data<Arc<CityService>>
) -> impl Responder {
    match city_service.find_all().await {
        Ok(cities) => HttpResponse::Ok().json(cities),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}