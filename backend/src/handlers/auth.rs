use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{db, errors};

#[derive(Deserialize)]
struct SignInBody {
    email: String,
    password: String,
}

async fn sign_in(
    pool: web::Data<PgPool>,
    body: web::Json<SignInBody>,
) -> Result<HttpResponse, errors::Error> {
    let user = db::users::get_user_by_email(&pool, body.email.clone()).await?;
    Ok(HttpResponse::Created().json(user))
}

async fn sign_out() -> impl Responder {
    HttpResponse::BadRequest().body("Bad request dude")
}

async fn me() -> impl Responder {
    HttpResponse::Unauthorized().body("Not authorized")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/signIn").route(web::post().to(sign_in)));
    cfg.service(web::resource("/signOut").route(web::post().to(sign_out)));
    cfg.service(web::resource("/me").route(web::get().to(me)));
}
