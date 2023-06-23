use crate::errors;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub photo_url: Option<String>,
    pub biography: Option<String>,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub photo_url: Option<String>,
    pub biography: Option<String>,
}

pub struct UserUpdate {
    pub first_name: String,
    pub last_name: String,
    pub email_verified: bool,
    pub photo_url: Option<String>,
    pub biography: String,
}

pub async fn insert_user(pool: &PgPool, new_user: NewUser) -> Result<User, errors::Error> {
    sqlx::query_as!(
        User,
        "
             INSERT INTO \"user\" (first_name, last_name, email, photo_url)
             VALUES ($1, $2, $3, $4)
             RETURNING *;
         ",
        new_user.first_name,
        new_user.last_name,
        new_user.email,
        new_user.photo_url,
    )
    .fetch_one(pool)
    .await
    .map_err(|err| errors::Error::DatabaseError(err.to_string()))
}

pub async fn update_user(
    pool: &PgPool,
    user_id: uuid::Uuid,
    update: UserUpdate,
) -> Result<User, errors::Error> {
    sqlx::query_as!(
        User,
        "
            UPDATE \"user\"
            SET
                updated_at = now(),
                first_name = $1,
                last_name = $2,
                photo_url = $3,
                email_verified = $4,
                biography = $5
            WHERE id = $6
            RETURNING *;
        ",
        update.first_name,
        update.last_name,
        update.photo_url,
        update.email_verified,
        update.biography,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|err| errors::Error::from_sqlx(err, "User"))
}

pub async fn get_user_by_email(pool: &PgPool, email: String) -> Result<User, errors::Error> {
    sqlx::query_as!(
        User,
        "
            SELECT * FROM \"user\"
            WHERE email = $1;
        ",
        email
    )
    .fetch_one(pool)
    .await
    .map_err(|err| errors::Error::from_sqlx(err, "User"))
}
