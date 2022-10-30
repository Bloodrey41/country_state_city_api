use actix_web::{HttpServer, App};
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;
use geo::api::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("host").unwrap_or(String::from("0.0.0.0"));

    let default_port = 8000;

    let port = env::var("PORT").unwrap_or(default_port.to_string()).parse().unwrap_or(default_port);

    HttpServer::new(move || {
        //let allowed_origin = env::var("ALLOWED_ORIGIN").unwrap_or("http://localhost:3000".to_owned());

        //let allowed_origin = &allowed_origin[..];

        //let cors = Cors::default().allowed_origin(allowed_origin);
        let cors = Cors::default();

        App::new()
            .wrap(cors)
            .configure(init_country_api)
            .configure(init_state_api)
            .configure(init_city_api)
    })
    .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
