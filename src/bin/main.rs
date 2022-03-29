use actix_web::{HttpServer, App};
use actix_cors::Cors;
use std::env;
use geo::api::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let default_port = 8080;

    let port = env::var("PORT").unwrap_or(default_port.to_string()).parse().unwrap_or(default_port);

    HttpServer::new(move || {
        let allowed_origin = env::var("ALLOWED_ORIGIN").unwrap_or(String::from("http://localhost:3000"));

        let allowed_origin = &allowed_origin[..];

        let cors = Cors::default().allowed_origin(allowed_origin);

        App::new()
            .wrap(cors)
            .configure(init_country_api)
            .configure(init_state_api)
            .configure(init_city_api)
    })
    .bind(("127.0.0.1", port))?
        .run()
        .await
}
