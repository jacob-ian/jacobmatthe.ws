use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx;
use sqlx::PgPool;

use crate::errors;

#[derive(Serialize)]
pub struct Post {
    id: uuid::Uuid,
    author_id: uuid::Uuid,
    stub: String,
    title: String,
    description: String,
    content: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    published_at: Option<DateTime<Utc>>,
}

pub async fn get_posts(pool: &PgPool) -> Result<Vec<Post>, errors::Error> {
    sqlx::query_as!(
        Post,
        "
            SELECT id,author_id,stub,title,description,content,created_at,updated_at,published_at 
            FROM \"post\"
            WHERE deleted_at IS NULL;
        ",
    )
    .fetch_all(pool)
    .await
    .map_err(|err| errors::Error::from_sqlx(err, "Post"))
}

pub async fn get_post_by_stub(pool: &PgPool, stub: String) -> Result<Post, errors::Error> {
    sqlx::query_as!(
        Post,
        "
            SELECT id,author_id,stub,title,description,content,created_at,updated_at,published_at 
            FROM \"post\"
            WHERE
                deleted_at IS NULL
                AND stub = $1;
        ",
        stub
    )
    .fetch_one(pool)
    .await
    .map_err(|err| errors::Error::from_sqlx(err, "Post"))
}
