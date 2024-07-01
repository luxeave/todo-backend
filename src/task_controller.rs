use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::data_validator;
use crate::database;
use crate::error_handler;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: Option<i64>,
    pub title: String,
    pub context: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub expected_end_date: Option<String>,
    pub tags: Vec<String>,
    pub notes: Option<String>,
    pub status: String,
}

pub async fn get_tasks() -> impl Responder {
    let db = database::get_connection();
    match database::get_tasks(&db) {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(e) => error_handler::handle_error(e),
    }
}

pub async fn create_task(task: web::Json<Task>) -> impl Responder {
    println!("Received task: {:?}", task);
    if let Err(e) = data_validator::validate_task(&task) {
        println!("Validation error: {}", e);
        return error_handler::handle_error(e);
    }

    let db = database::get_connection();
    match database::create_task(&db, &task) {
        Ok(id) => {
            let mut created_task = task.into_inner();
            created_task.id = Some(id);
            HttpResponse::Created().json(created_task)
        },
        Err(e) => {
            println!("Database error: {}", e);
            error_handler::handle_error(e)
        }
    }
}

pub async fn update_task(id: web::Path<i64>, task: web::Json<Task>) -> impl Responder {
    if let Err(e) = data_validator::validate_task(&task) {
        return error_handler::handle_error(e);
    }

    let db = database::get_connection();
    match database::update_task(&db, *id, &task) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => error_handler::handle_error(e),
    }
}