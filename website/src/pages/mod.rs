use actix_web::web::{self, ServiceConfig};

mod blog;
mod home;
mod now;
mod post;

/// Configure the webpage routes
pub fn config(cfg: &mut ServiceConfig) {
    cfg.route("/blog", web::get().to(blog::blog));
    cfg.route("/now", web::get().to(now::now));
    cfg.route("/", web::get().to(home::home));
    cfg.route("/{stub}", web::get().to(post::post));
}
