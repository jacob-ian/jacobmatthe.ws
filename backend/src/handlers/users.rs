use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::passwords,
    db::{self, users::UserUpdate},
    errors::Error,
};

#[derive(Deserialize)]
struct ChangePasswordBody {
    old_password: String,
    new_password: String,
}

#[derive(Serialize)]
struct ChangePasswordResponse {
    message: String,
}

#[derive(Deserialize)]
struct VerifyEmailBody {
    code: String,
}

#[derive(Serialize)]
struct VerifyEmailResponse {
    message: String,
}

async fn update_user(
    session: Session,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<UserUpdate>,
) -> Result<HttpResponse, Error> {
    let signed_in_user = if let Ok(Some(id)) = session.get::<Uuid>("user_id") {
        id
    } else {
        return Err(Error::UnauthorizedError(format!("Not signed in")));
    };

    let user_id = path.into_inner();
    let update = body.into_inner();

    if signed_in_user != user_id {
        return Err(Error::ForbiddenError(format!("Forbidden")));
    }

    let user = db::users::update_user(&pool, user_id, update).await?;
    Ok(HttpResponse::Ok().json(user))
}

async fn change_password(
    session: Session,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ChangePasswordBody>,
) -> Result<HttpResponse, Error> {
    let signed_in_user = if let Ok(Some(id)) = session.get::<Uuid>("user_id") {
        id
    } else {
        return Err(Error::UnauthorizedError(format!("Not signed in")));
    };

    let user_id = path.into_inner();
    let payload = body.into_inner();

    if signed_in_user != user_id {
        return Err(Error::ForbiddenError(format!("Forbidden")));
    }

    if payload.new_password.len() < 12 {
        return Err(Error::BadRequestError(format!(
            "Password must have at least 12 characters"
        )));
    }

    passwords::verify_password(&pool, user_id, payload.old_password).await?;
    passwords::set_password(&pool, user_id, payload.new_password).await?;

    Ok(HttpResponse::Ok().json(ChangePasswordResponse {
        message: format!("Password changed"),
    }))
}

async fn verify_email_code(
    session: Session,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<VerifyEmailBody>,
) -> Result<HttpResponse, Error> {
    let signed_in_user = if let Ok(Some(id)) = session.get::<Uuid>("user_id") {
        id
    } else {
        return Err(Error::UnauthorizedError(format!("Not signed in")));
    };

    let user_id = path.into_inner();
    let payload = body.into_inner();

    if signed_in_user != user_id {
        return Err(Error::ForbiddenError(format!("Forbidden")));
    }

    let verification =
        db::email_verifications::get_verification_by_code(&pool, user_id, payload.code).await;

    if !verification.is_ok() {
        return Err(Error::BadRequestError(format!("Invalid code")));
    }
    db::users::set_email_verified(&pool, user_id).await?;
    Ok(HttpResponse::Ok().json(VerifyEmailResponse {
        message: format!("Email verified"),
    }))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/{user_id}").route(web::put().to(update_user)));
    cfg.service(web::resource("/{user_id}/password").route(web::put().to(change_password)));
    cfg.service(web::resource("/{user_id}/verify").route(web::post().to(verify_email_code)));
}
