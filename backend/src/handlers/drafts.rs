use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::{
    db::{
        self,
        posts::{NewPost, UpdatePost},
    },
    errors,
};

/// Gets the drafts
pub async fn get_drafts(pool: web::Data<PgPool>) -> Result<HttpResponse, errors::Error> {
    let drafts = db::posts::get_drafts(&pool).await?;
    Ok(HttpResponse::Ok().json(drafts))
}

/// Gets a draft post by ID
pub async fn get_draft_by_id(
    pool: web::Data<PgPool>,
    path_id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, errors::Error> {
    let draft_id = path_id.into_inner();
    let draft = db::posts::get_draft_by_id(&pool, draft_id).await?;
    Ok(HttpResponse::Ok().json(draft))
}

/// Creates a draft
pub async fn create_draft(
    pool: web::Data<PgPool>,
    data: web::Json<NewPost>,
) -> Result<HttpResponse, errors::Error> {
    let new_post = data.into_inner();
    let new_draft = db::posts::create_post(&pool, new_post).await?;
    Ok(HttpResponse::Created().json(new_draft))
}

/// Updates a draft by its ID
pub async fn update_draft_by_id(
    pool: web::Data<PgPool>,
    path_id: web::Path<uuid::Uuid>,
    data: web::Json<UpdatePost>,
) -> Result<HttpResponse, errors::Error> {
    let update = data.into_inner();
    let id = path_id.into_inner();
    let draft = db::posts::update_post(&pool, id, update).await?;
    Ok(HttpResponse::Created().json(draft))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(get_drafts));
    cfg.route("", web::post().to(create_draft));
    cfg.route("/{id}", web::get().to(get_draft_by_id));
    cfg.route("/{id}", web::put().to(update_draft_by_id));
}
