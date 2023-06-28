use actix_web::{HttpResponse, Responder};

pub mod auth;
pub mod drafts;
pub mod posts;
pub mod uploads;
pub mod users;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}
