use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use chrono::{DateTime, Utc};
use sqlx::{self, PgPool};
use uuid::Uuid;

use crate::errors::{self, Error};

pub struct UserCredential {
    id: Uuid,
    user_id: Uuid,
    password_hash: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub async fn get_password_hash_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<String, Error> {
    let res = sqlx::query!(
        "
        SELECT password_hash
        FROM \"user_credential\"
        WHERE 
            user_id = $1;
        ",
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| errors::Error::from_sqlx(e, "Credential"))?;
    Ok(res.password_hash)
}

pub async fn create_user_credential(
    pool: &PgPool,
    user_id: Uuid,
    password: String,
) -> Result<UserCredential, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| Error::InternalServerError(format!("Could not register user")))?;

    sqlx::query_as!(
        UserCredential,
        "
            INSERT INTO \"user_credential\" 
            (user_id, password_hash)
            VALUES ($1, $2)
            RETURNING *;
        ",
        user_id,
        hash.to_string()
    )
    .fetch_one(pool)
    .await
    .map_err(|e| Error::from_sqlx(e, "Credential"))
}
