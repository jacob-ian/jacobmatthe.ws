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
    pub file_type: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub uploaded_at: Option<DateTime<Utc>>,
}

pub struct NewUpload {
    pub file_name: String,
    pub file_type: String,
}

pub async fn create_upload(pool: &PgPool, upload: NewUpload) -> Result<Upload, Error> {
    sqlx::query_as!(
        Upload,
        "
            INSERT INTO \"upload\" (file_name, file_type)
            VALUES ($1, $2)
            RETURNING id, file_name, file_type, created_by, created_at, updated_at, uploaded_at;
        ",
        upload.file_name,
        upload.file_type
    )
    .fetch_one(pool)
    .await
    .map_err(|e| Error::from_sqlx(e, "Upload"))
}

pub async fn get_uploads(pool: &PgPool) -> Result<Vec<Upload>, Error> {
    sqlx::query_as!(
        Upload,
        "
            SELECT id, file_name, file_type, created_by, created_at, updated_at, uploaded_at
            FROM \"upload\"
            WHERE 
                deleted_at IS NULL
                AND uploaded_at IS NULL;
        "
    )
    .fetch_all(pool)
    .await
    .map_err(|e| Error::from_sqlx(e, "Upload"))
}

pub async fn get_uploads_by_post_id(pool: &PgPool, post_id: Uuid) -> Result<Vec<Upload>, Error> {
    sqlx::query_as!(
        Upload,
        "
            SELECT 
                upload.id, 
                upload.file_name, 
                upload.file_type, 
                upload.created_by, 
                upload.created_at, 
                upload.updated_at, 
                upload.uploaded_at
            FROM \"upload\"
            LEFT JOIN \"post_upload\" on upload_id = upload.id
            WHERE 
                \"post_upload\".post_id = $1
                AND deleted_at IS NULL
                AND uploaded_at IS NULL;
        ",
        post_id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| Error::from_sqlx(e, "Upload"))
}

pub async fn get_upload_by_id(pool: &PgPool, id: Uuid) -> Result<Upload, Error> {
    sqlx::query_as!(
        Upload,
        "
            SELECT id, file_name, file_type, created_by, created_at, updated_at, uploaded_at
            FROM \"upload\"
            WHERE
                id = $1
                AND deleted_at IS NULL;
        ",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| Error::from_sqlx(e, "Upload"))
}

pub async fn get_incoming_upload_by_id(pool: &PgPool, id: Uuid) -> Result<Upload, Error> {
    sqlx::query_as!(
        Upload,
        "
            SELECT id, file_name, file_type, created_by, created_at, updated_at, uploaded_at
            FROM \"upload\"
            WHERE
                id = $1
                AND uploaded_at IS NULL
                AND deleted_at IS NULL;
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
                id = $1
                AND deleted_at IS NULL;
        ",
        id
    )
    .execute(pool)
    .await
    .map_err(|e| Error::from_sqlx(e, "Upload"))?;
    Ok(())
}

pub async fn delete_by_id(pool: &PgPool, id: Uuid) -> Result<(), Error> {
    sqlx::query!(
        "
            UPDATE \"upload\"
            SET
                deleted_at = now()
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
