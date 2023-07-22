use actix_web::web::{self, ServiceConfig};

mod about;
mod blog;
mod home;
mod post;

/// Configure the webpage routes
pub fn config(cfg: &mut ServiceConfig) {
    cfg.route("/blog", web::get().to(blog::blog));
    cfg.route("/about", web::get().to(about::about));
    cfg.route("/", web::get().to(home::home));
    cfg.route("/{stub}", web::get().to(post::post));
}
