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

pub async fn get_password_hash_by_user_id<'a>(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<String, Error> {
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
