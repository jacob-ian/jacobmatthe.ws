use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use include_dir::{include_dir, Dir};

static FILES: Dir = include_dir!("./static");

async fn serve_file(name: web::Path<String>) -> impl Responder {
    return match FILES.get_file(name.into_inner()) {
        Some(f) => HttpResponse::Ok().body(f.contents()),
        None => HttpResponse::NotFound().into(),
    };
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.route("/static/{name:.*}", web::get().to(serve_file));
}
