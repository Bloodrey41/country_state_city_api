use actix_web::{get, web::{Path, ServiceConfig}, HttpResponse};
use crate::repository::country::CountryRepository;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(get_countries);
    cfg.service(get_country);
}

#[get("/countries")]
pub async fn get_countries() -> HttpResponse {
    let country_repository = CountryRepository::new();

    let result = country_repository.find();
    
    match result {
        Ok(countries) => HttpResponse::Ok().json(countries),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

#[get("/countries/{country_id}")]
pub async fn get_country(country_id: Path<u32>) -> HttpResponse {
    let country_repository = CountryRepository::new();

    let result = country_repository.find_by_id(country_id.into_inner());

    match result {
        Ok(row) => {
            match row {
                Some(country) => HttpResponse::Ok().json(country),
                None => HttpResponse::NotFound().body("Country not found"),
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}
