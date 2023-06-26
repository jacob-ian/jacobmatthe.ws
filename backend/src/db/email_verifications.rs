use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::Error;

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
