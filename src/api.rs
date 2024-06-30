use actix_web::{web, Scope};
use crate::task_controller;

pub fn config() -> Scope {
    web::scope("/api")
        .service(web::resource("/tasks").route(web::get().to(task_controller::get_tasks)))
        .service(web::resource("/task").route(web::post().to(task_controller::create_task)))
        .service(web::resource("/task/{id}").route(web::put().to(task_controller::update_task)))
}