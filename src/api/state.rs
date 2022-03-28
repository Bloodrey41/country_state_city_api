use actix_web::{get, web::{Path, ServiceConfig}, HttpResponse};
use crate::repository::state::StateRepository;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(get_states);
    cfg.service(get_state);
    cfg.service(get_country_states);
}

#[get("/states")]
pub async fn get_states() -> HttpResponse {
    let state_repository = StateRepository::new();

    let result = state_repository.find();
    
    match result {
        Ok(states) => HttpResponse::Ok().json(states),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

#[get("/states/{state_id}")]
pub async fn get_state(state_id: Path<u32>) -> HttpResponse {
    let state_repository = StateRepository::new();

    let result = state_repository.find_by_id(state_id.into_inner());

    match result {
        Ok(row) => {
            match row {
                Some(state) => HttpResponse::Ok().json(state),
                None => HttpResponse::NotFound().body("State not found"),
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

#[get("/states/country/{country_id}")]
pub async fn get_country_states(country_id: Path<u32>) -> HttpResponse {
    let state_repository = StateRepository::new();

    let result = state_repository.find_by_country(country_id.into_inner());
    
     match result {
        Ok(row) => {
            match row {
                Some(states) => HttpResponse::Ok().json(states),
                None => HttpResponse::NotFound().body("Country not found"),
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}
