use actix_web::{get, web::{Path, ServiceConfig}, HttpResponse};
use crate::repository::city::CityRepository;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(get_cities);
    cfg.service(get_city);
    cfg.service(get_state_cities);
    cfg.service(get_country_cities);
}

#[get("/cities")]
pub async fn get_cities() -> HttpResponse {
    let city_repository = CityRepository::new();

    let result = city_repository.find();
    
    match result {
        Ok(cities) => HttpResponse::Ok().json(cities),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

#[get("/cities/{city_id}")]
pub async fn get_city(city_id: Path<u32>) -> HttpResponse {
    let city_repository = CityRepository::new();

    let result = city_repository.find_by_id(city_id.into_inner());

    match result {
        Ok(row) => {
            match row {
                Some(city) => HttpResponse::Ok().json(city),
                None => HttpResponse::NotFound().body("City not found"),
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

#[get("/cities/state/{state_id}")]
pub async fn get_state_cities(state_id: Path<u32>) -> HttpResponse {
    let city_repository = CityRepository::new();

    let result = city_repository.find_by_state(state_id.into_inner());
    
     match result {
        Ok(value) => {
            match value {
                Some(cities) => HttpResponse::Ok().json(cities),
                None => HttpResponse::NotFound().body("State not found"),
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

#[get("/cities/country/{country_id}")]
pub async fn get_country_cities(country_id: Path<u32>) -> HttpResponse {
    let city_repository = CityRepository::new();

    let result = city_repository.find_by_country(country_id.into_inner());
    
     match result {
        Ok(value) => {
            match value {
                Some(cities) => HttpResponse::Ok().json(cities),
                None => HttpResponse::NotFound().body("Country not found"),
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}
