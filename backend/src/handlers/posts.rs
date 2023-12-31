use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{db, errors};

#[derive(Deserialize)]
pub struct GetPostsFilter {
    limit: Option<i64>,
}

/// Gets published posts
async fn get_posts(
    pool: web::Data<PgPool>,
    query: web::Query<GetPostsFilter>,
) -> Result<HttpResponse, errors::Error> {
    let posts = db::posts::get_posts(&pool, query.limit).await?;
    Ok(HttpResponse::Ok().json(posts))
}

/// Gets a published post by stub
async fn get_post_by_stub(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, errors::Error> {
    let stub = path.into_inner();
    let post = db::posts::get_post_by_stub(&pool, stub).await?;
    Ok(HttpResponse::Ok().json(post))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(get_posts))
        .route("/{stub}", web::get().to(get_post_by_stub));
}
