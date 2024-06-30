mod api;
mod task_controller;
mod data_validator;
mod database;
mod error_handler;

use actix_web::{App, HttpServer};
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the database
    database::initialize_database().expect("Failed to initialize database");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(api::config())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}