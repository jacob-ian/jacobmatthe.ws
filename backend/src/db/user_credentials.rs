use chrono::{DateTime, Utc};
use sqlx::{self, PgPool};
use uuid::Uuid;

use crate::errors::{self, Error};

#[allow(dead_code)]
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

pub async fn upsert_user_credential(
    pool: &PgPool,
    user_id: Uuid,
    password_hash: String,
) -> Result<UserCredential, Error> {
    sqlx::query_as!(
        UserCredential,
        "
            INSERT INTO \"user_credential\" (user_id, password_hash)
            VALUES ($1, $2)
            ON CONFLICT (user_id)
            DO
                UPDATE SET password_hash = $2
            RETURNING *;
        ",
        user_id,
        password_hash
    )
    .fetch_one(pool)
    .await
    .map_err(|e| Error::from_sqlx(e, "Credential"))
}
