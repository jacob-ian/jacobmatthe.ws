use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2,
};
use crypto::password_hash::PasswordHash;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db::{self, users::User},
    errors::Error,
};

/// Verifies a user's password by an email address
pub async fn verify_password_by_email(
    pool: &PgPool,
    email: String,
    password: String,
) -> Result<User, Error> {
    let user = db::users::get_user_by_email(pool, email)
        .await
        .map_err(|e| match e {
            Error::NotFoundError(_) => Error::BadRequestError(format!("Invalid email or password")),
            _ => Error::InternalServerError(format!("Verification failed")),
        })?;
    verify_password(pool, user.id, password)
        .await
        .map_err(|e| match e {
            Error::BadRequestError(_) => {
                Error::BadRequestError(format!("Invalid email or password"))
            }
            _ => Error::InternalServerError(format!("Verification failed")),
        })?;
    Ok(user)
}

/// Verifies a user's password given the user's id
pub async fn verify_password(pool: &PgPool, user_id: Uuid, password: String) -> Result<(), Error> {
    let hash = db::user_credentials::get_password_hash_by_user_id(&pool, user_id)
        .await
        .map_err(|e| match e {
            Error::NotFoundError(_) => Error::BadRequestError(format!("Incorrect password")),
            _ => Error::InternalServerError(format!("Verification failed")),
        })?;

    let hash = PasswordHash::new(&hash)
        .map_err(|_| Error::InternalServerError(format!("Verification failed")))?;

    hash.verify_password(&[&Argon2::default()], password)
        .map_err(|e| match e {
            argon2::password_hash::Error::Password => {
                Error::BadRequestError(format!("Incorrect password"))
            }
            _ => Error::InternalServerError(format!("Verification failed")),
        })?
}

/// Sets a user's password to the one provided
pub async fn set_password(pool: &PgPool, user_id: Uuid, password: String) -> Result<(), Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = PasswordHash::generate(&Argon2::default(), password, &salt)
        .map_err(|e| Error::InternalServerError(format!("Could not change password")))?;
    db::user_credentials::upsert_user_credential(pool, user_id, hash.to_string()).await?;
}
