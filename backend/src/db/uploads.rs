use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::Error;

#[allow(dead_code)]
#[derive(Serialize)]
pub struct Upload {
    pub id: Uuid,
    pub file_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub uploaded_at: Option<DateTime<Utc>>,
}

pub async fn create_upload(pool: &PgPool, file_name: String) -> Result<Upload, Error> {
    sqlx::query_as!(
        Upload,
        "
            INSERT INTO \"upload\" (file_name)
            VALUES ($1)
            RETURNING id, file_name, created_at, updated_at, uploaded_at;
        ",
        file_name
    )
    .fetch_one(pool)
    .await
    .map_err(|e| Error::from_sqlx(e, "Upload"))
}

pub async fn get_incoming_upload_by_id(pool: &PgPool, id: Uuid) -> Result<Upload, Error> {
    sqlx::query_as!(
        Upload,
        "
            SELECT id, file_name, created_at, updated_at, uploaded_at
            FROM \"upload\"
            WHERE
                id = $1
                AND uploaded_at IS NULL;
        ",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| Error::from_sqlx(e, "Upload"))
}

pub async fn set_as_uploaded_by_id(pool: &PgPool, id: Uuid) -> Result<(), Error> {
    sqlx::query_as!(
        Upload,
        "
            UPDATE \"upload\"
            SET 
                uploaded_at = now()
            WHERE
                id = $1;
        ",
        id
    )
    .execute(pool)
    .await
    .map_err(|e| Error::from_sqlx(e, "Upload"))?;
    Ok(())
}
