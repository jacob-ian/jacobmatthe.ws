use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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

#[derive(Deserialize)]
pub struct NewPost {
    author_id: uuid::Uuid,
    stub: String,
    title: String,
    description: String,
    content: String,
}

#[derive(Deserialize)]
pub struct UpdatePost {
    stub: String,
    title: String,
    description: String,
    content: String,
    published_at: Option<DateTime<Utc>>,
}

/// Gets all published posts
pub async fn get_posts(pool: &PgPool) -> Result<Vec<Post>, errors::Error> {
    sqlx::query_as!(
        Post,
        "
            SELECT id,author_id,stub,title,description,content,created_at,updated_at,published_at 
            FROM \"post\"
            WHERE 
                deleted_at IS NULL
                AND published_at IS NOT NULL
            ORDER BY published_at DESC;
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
pub async fn get_draft_by_id(pool: &PgPool, id: uuid::Uuid) -> Result<Post, errors::Error> {
    sqlx::query_as!(
        Post,
        "
            SELECT id,author_id,stub,title,description,content,created_at,updated_at,published_at 
            FROM \"post\"
            WHERE
                deleted_at IS NULL
                AND published_at IS NULL
                AND id = $1;
        ",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|err| errors::Error::from_sqlx(err, "Post"))
}

/// Creates a new post
pub async fn create_post(pool: &PgPool, post: NewPost) -> Result<Post, errors::Error> {
    sqlx::query_as!(
        Post,
        "
            INSERT INTO \"post\" (author_id, stub, title, description, content)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, author_id, stub, title, description, content, created_at, updated_at, published_at;
        ",
        post.author_id,
        post.stub,
        post.title,
        post.description,
        post.content
    )
    .fetch_one(pool)
    .await
    .map_err(|err| errors::Error::from_sqlx(err, "Post"))
}

/// Updates a post by ID
pub async fn update_post(
    pool: &PgPool,
    id: uuid::Uuid,
    update: UpdatePost,
) -> Result<Post, errors::Error> {
    sqlx::query_as!(
        Post,
        "
            UPDATE \"post\" 
            SET 
                updated_at = now(),
                stub = $1,
                title = $2,
                description = $3,
                content = $4,
                published_at = $5
            WHERE
                deleted_at IS NULL
                AND id = $6
            RETURNING id, author_id, stub, title, description, content, created_at, updated_at, published_at;
        ",
        update.stub,
        update.title,
        update.description,
        update.content,
        update.published_at,
        id,
    )
    .fetch_one(pool)
    .await
    .map_err(|err| errors::Error::from_sqlx(err, "Post"))
}
