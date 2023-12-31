use crate::errors::{self, Error};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Deserialize)]
pub struct UserUpdate {
    pub first_name: String,
    pub last_name: String,
    pub photo_url: Option<String>,
    pub biography: Option<String>,
}

pub async fn create_user(pool: &PgPool, new_user: NewUser) -> Result<User, errors::Error> {
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
                biography = $4
            WHERE id = $5
            RETURNING *;
        ",
        update.first_name,
        update.last_name,
        update.photo_url,
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

pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> Result<User, errors::Error> {
    sqlx::query_as!(
        User,
        "
            SELECT * FROM \"user\"
            WHERE id= $1;
        ",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|err| errors::Error::from_sqlx(err, "User"))
}

pub async fn set_email_verified(pool: &PgPool, user_id: Uuid) -> Result<(), Error> {
    sqlx::query!(
        "
            UPDATE \"user\"
            SET
                email_verified = true
            WHERE
                id = $1;
        ",
        user_id
    )
    .execute(pool)
    .await
    .map_err(|err| errors::Error::from_sqlx(err, "User"))?;
    Ok(())
}

pub async fn is_email_verified(pool: &PgPool, user_id: Uuid) -> Result<bool, Error> {
    let user = sqlx::query!(
        "
            SELECT email_verified
            FROM \"user\"
            WHERE 
                id = $1;
        ",
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| Error::from_sqlx(e, "User"))?;
    return Ok(user.email_verified);
}
