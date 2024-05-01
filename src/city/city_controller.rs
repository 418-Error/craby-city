use crate::city::city_service::CityService;
use crate::city::model::CreateCity;
use actix_web::{get, post, web, HttpResponse, Responder};
use tracing::{error, info};
use std::sync::Arc;

#[get("/cities")]
pub async fn get_all_cities(city_service: web::Data<Arc<CityService>>) -> impl Responder {
    info!("Getting all cities");
    match city_service.find_all().await {
        Ok(cities) => HttpResponse::Ok().json(cities),
        Err(e) => {
            error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/cities")]
pub async fn create_city(
    city_service: web::Data<Arc<CityService>>,
    city: web::Json<CreateCity>,
) -> impl Responder {
    info!("Creating city");
    match city_service.create(city.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
