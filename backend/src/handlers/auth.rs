use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db::{self},
    errors,
};

#[derive(Deserialize)]
struct SignInBody {
    email: String,
    password: String,
}

/// Starts a new session for a user with provided credentials
async fn sign_in(
    session: Session,
    pool: web::Data<PgPool>,
    body: web::Json<SignInBody>,
) -> Result<HttpResponse, errors::Error> {
    if let Ok(Some(_)) = session.get::<uuid::Uuid>("user_id") {
        return Err(errors::Error::BadRequestError(format!("Already signed in")));
    }
    let details = body.into_inner();
    let user = db::users::get_user_by_email(&pool, details.email.clone()).await?;
    session
        .insert("user_id", user.id.clone())
        .map_err(|e| errors::Error::InternalServerError(e.to_string()))?;
    Ok(HttpResponse::Ok().json(user))
}

/// Signs out the currently signed in user or returns 401
async fn sign_out(session: Session) -> Result<HttpResponse, errors::Error> {
    if let Ok(None) = session.get::<uuid::Uuid>("user_id") {
        return Err(errors::Error::UnauthorizedError(format!("Not signed in")));
    }
    session.clear();
    Ok(HttpResponse::NoContent().into())
}

/// Gets the currently signed in user or returns 401
async fn me(session: Session, pool: web::Data<PgPool>) -> Result<HttpResponse, errors::Error> {
    let opt = session
        .get::<Uuid>("user_id")
        .map_err(|e| errors::Error::InternalServerError(e.to_string()))?;

    let user_id = if let Some(id) = opt {
        Ok(id)
    } else {
        Err(errors::Error::UnauthorizedError(format!("Not signed in")))
    }?;

    let user = db::users::get_user_by_id(&pool, user_id).await?;
    Ok(HttpResponse::Ok().json(user))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/signIn").route(web::post().to(sign_in)));
    cfg.service(web::resource("/signOut").route(web::post().to(sign_out)));
    cfg.service(web::resource("/me").route(web::get().to(me)));
}
