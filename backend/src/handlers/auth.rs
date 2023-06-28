use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::{passwords, verification},
    config::Config,
    db::{self, users::NewUser},
    errors::{self, Error},
};

#[derive(Deserialize)]
struct SignInBody {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct RegisterBody {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    photo_url: Option<String>,
    biography: Option<String>,
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

    let credentials = body.into_inner();
    let user =
        passwords::verify_password_by_email(&pool, credentials.email, credentials.password).await?;

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

/// Registers a user
async fn register_user(
    session: Session,
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    body: web::Json<RegisterBody>,
) -> Result<HttpResponse, errors::Error> {
    if let Ok(Some(_)) = session.get::<uuid::Uuid>("user_id") {
        return Err(Error::BadRequestError(format!("Already signed in")));
    }
    let new_user = body.into_inner();
    if let Some(whitelist) = &config.auth.whitelist {
        if !whitelist.contains(&new_user.email) {
            return Err(Error::ForbiddenError(format!("Forbidden")));
        }
    }
    if let Ok(_) = db::users::get_user_by_email(&pool, new_user.email.clone()).await {
        return Err(Error::BadRequestError(format!(
            "User with this email already exists"
        )));
    }

    if new_user.password.len() < 12 {
        return Err(Error::BadRequestError(format!(
            "Password must have at least 12 characters"
        )));
    }

    let user = db::users::create_user(
        &pool,
        NewUser {
            email: new_user.email,
            first_name: new_user.first_name,
            last_name: new_user.last_name,
            biography: new_user.biography,
            photo_url: new_user.photo_url,
        },
    )
    .await?;

    passwords::set_password(&pool, user.id, new_user.password).await?;
    verification::send_verification_email(&config, &pool, user.id).await?;

    session.insert("user_id", user.id).unwrap_or(());
    Ok(HttpResponse::Created().json(user))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/register").route(web::post().to(register_user)));
    cfg.service(web::resource("/signIn").route(web::post().to(sign_in)));
    cfg.service(web::resource("/signOut").route(web::post().to(sign_out)));
    cfg.service(web::resource("/me").route(web::get().to(me)));
}
