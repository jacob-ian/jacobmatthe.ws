use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::Error;

#[allow(dead_code)]
pub struct EmailVerification {
    id: Uuid,
    user_id: Uuid,
    code: String,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

pub async fn get_verification_by_code(
    pool: &PgPool,
    user_id: Uuid,
    code: String,
) -> Result<EmailVerification, Error> {
    sqlx::query_as!(
        EmailVerification,
        "
            SELECT *
            FROM \"email_verification\"
            WHERE
                expires_at > now()
                AND user_id = $1
                AND code = $2;
        ",
        user_id,
        code
    )
    .fetch_one(pool)
    .await
    .map_err(|e| Error::from_sqlx(e, "Email Verification"))
}

pub async fn save_verification_code(
    pool: &PgPool,
    user_id: Uuid,
    code: String,
) -> Result<EmailVerification, Error> {
    sqlx::query!(
        "
            UPDATE \"email_verification\"
            SET
                expires_at = now()
            WHERE
                user_id = $1
        ",
        user_id
    )
    .execute(pool)
    .await
    .map_err(|_| Error::InternalServerError(format!("Failed to invalidate previous codes")))?;

    sqlx::query_as!(
        EmailVerification,
        "
            INSERT INTO \"email_verification\" (user_id, code)
            VALUES ($1, $2)
            RETURNING *;
        ",
        user_id,
        code
    )
    .fetch_one(pool)
    .await
    .map_err(|e| Error::from_sqlx(e, "Email Verification"))
}
