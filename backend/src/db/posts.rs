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

/// Gets all posts (drafts and published)
pub async fn get_posts(pool: &PgPool) -> Result<Vec<Post>, errors::Error> {
    sqlx::query_as!(
        Post,
        "
            SELECT id,author_id,stub,title,description,content,created_at,updated_at,published_at 
            FROM \"post\"
            WHERE deleted_at IS NULL
            ORDER BY created_at DESC;
        ",
    )
    .fetch_all(pool)
    .await
    .map_err(|err| errors::Error::from_sqlx(err, "Post"))
}

/// Gets all published posts
pub async fn get_published_posts(pool: &PgPool) -> Result<Vec<Post>, errors::Error> {
    sqlx::query_as!(
        Post,
        "
            SELECT id,author_id,stub,title,description,content,created_at,updated_at,published_at 
            FROM \"post\"
            WHERE
                deleted_at IS NULL
                AND published_at IS NOT NULL
            ORDER BY
                published_at DESC;
        ",
    )
    .fetch_all(pool)
    .await
    .map_err(|err| errors::Error::from_sqlx(err, "Post"))
}

/// Gets all drafts
pub async fn get_drafts(pool: &PgPool) -> Result<Vec<Post>, errors::Error> {
    sqlx::query_as!(
        Post,
        "
            SELECT id,author_id,stub,title,description,content,created_at,updated_at,published_at 
            FROM \"post\"
            WHERE
                deleted_at IS NULL
                AND published_at IS NULL
            ORDER BY
                created_at DESC;
        ",
    )
    .fetch_all(pool)
    .await
    .map_err(|err| errors::Error::from_sqlx(err, "Post"))
}

/// Gets a published post by stub
pub async fn get_post_by_stub(pool: &PgPool, stub: String) -> Result<Post, errors::Error> {
    sqlx::query_as!(
        Post,
        "
            SELECT id,author_id,stub,title,description,content,created_at,updated_at,published_at 
            FROM \"post\"
            WHERE
                deleted_at IS NULL
                AND published_at IS NOT NULL
                AND stub = $1;
        ",
        stub
    )
    .fetch_one(pool)
    .await
    .map_err(|err| errors::Error::from_sqlx(err, "Post"))
}

/// Gets a post by ID
pub async fn get_post_by_id(pool: &PgPool, id: uuid::Uuid) -> Result<Post, errors::Error> {
    sqlx::query_as!(
        Post,
        "
            SELECT id,author_id,stub,title,description,content,created_at,updated_at,published_at 
            FROM \"post\"
            WHERE
                deleted_at IS NULL
                AND id = $1;
        ",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|err| errors::Error::from_sqlx(err, "Post"))
}
