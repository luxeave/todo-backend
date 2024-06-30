use actix_web::HttpResponse;

pub fn handle_error<T: std::fmt::Display>(error: T) -> HttpResponse {
    HttpResponse::InternalServerError().json(format!("Error: {}", error))
}