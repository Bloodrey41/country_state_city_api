use actix_web::{HttpServer, App};
use geo::api::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .configure(init_country_api)
            .configure(init_state_api)
            .configure(init_city_api)
    })
    .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
