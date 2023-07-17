use actix_web::web::{self, ServiceConfig};

mod home;
mod test;

/// Configure the webpage routes
pub fn config(cfg: &mut ServiceConfig) {
    cfg.route("/", web::get().to(home::home));
    cfg.route("/page", web::get().to(test::page));
}
