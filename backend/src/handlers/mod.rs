use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

pub mod auth;
pub mod drafts;
pub mod posts;
pub mod uploads;
pub mod users;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.route("/", web::get().to(health_check));
    cfg.service(web::scope("/auth").configure(auth::config));
    cfg.service(web::scope("/users").configure(users::config));
    cfg.service(web::scope("/posts").configure(posts::config));
    cfg.service(web::scope("/drafts").configure(drafts::config));
    cfg.service(web::scope("/uploads").configure(uploads::config));
}
