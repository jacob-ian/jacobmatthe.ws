use actix_session::Session;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::Config,
    db::{
        self,
        posts::{NewPost, UpdatePost},
        uploads::NewUpload,
        users::is_email_verified,
    },
    errors::{self, Error},
    files,
};

use super::uploads::CreateUploadBody;

/// Gets the drafts
pub async fn get_drafts(
    session: Session,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, errors::Error> {
    if let Ok(None) = session.get::<Uuid>("user_id") {
        return Err(Error::UnauthorizedError(format!("Unauthorized")));
    }
    let drafts = db::posts::get_drafts(&pool).await?;
    Ok(HttpResponse::Ok().json(drafts))
}

/// Gets a draft post by ID
pub async fn get_draft_by_id(
    session: Session,
    pool: web::Data<PgPool>,
    path_id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, errors::Error> {
    if let Ok(None) = session.get::<Uuid>("user_id") {
        return Err(Error::UnauthorizedError(format!("Unauthorized")));
    }
    let draft_id = path_id.into_inner();
    let draft = db::posts::get_draft_by_id(&pool, draft_id).await?;
    Ok(HttpResponse::Ok().json(draft))
}

/// Creates a draft
pub async fn create_draft(
    session: Session,
    pool: web::Data<PgPool>,
    data: web::Json<NewPost>,
) -> Result<HttpResponse, errors::Error> {
    let user_id = if let Ok(Some(id)) = session.get::<Uuid>("user_id") {
        id
    } else {
        return Err(Error::UnauthorizedError(format!("Unauthorized")));
    };
    if !is_email_verified(&pool, user_id).await? {
        return Err(Error::ForbiddenError(format!("Email must be verified")));
    }
    let new_post = data.into_inner();
    let new_draft = db::posts::create_post(&pool, new_post).await?;
    Ok(HttpResponse::Created().json(new_draft))
}

/// Updates a draft by its ID
pub async fn update_draft_by_id(
    session: Session,
    pool: web::Data<PgPool>,
    path_id: web::Path<uuid::Uuid>,
    data: web::Json<UpdatePost>,
) -> Result<HttpResponse, errors::Error> {
    let user_id = if let Ok(Some(id)) = session.get::<Uuid>("user_id") {
        id
    } else {
        return Err(Error::UnauthorizedError(format!("Unauthorized")));
    };
    if !is_email_verified(&pool, user_id).await? {
        return Err(Error::ForbiddenError(format!("Email must be verified")));
    }
    let update = data.into_inner();
    let id = path_id.into_inner();
    let draft = db::posts::update_post(&pool, id, update).await?;
    Ok(HttpResponse::Created().json(draft))
}

pub async fn get_draft_uploads(
    session: Session,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    if let Ok(None) = session.get::<Uuid>("user_id") {
        return Err(Error::UnauthorizedError(format!("Unauthorized")));
    }
    let post_id = path.into_inner();
    let _post = db::posts::get_draft_by_id(&pool, post_id).await?;
    let uploads = db::uploads::get_uploads_by_post_id(&pool, post_id).await?;
    Ok(HttpResponse::Ok().json(uploads))
}

pub async fn create_draft_upload(
    session: Session,
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    path: web::Path<Uuid>,
    body: web::Json<CreateUploadBody>,
) -> Result<HttpResponse, Error> {
    if let Ok(None) = session.get::<Uuid>("user_id") {
        return Err(Error::UnauthorizedError(format!("Unauthorized")));
    }
    let post_id = path.into_inner();
    let payload = body.into_inner();
    let upload = files::create_file_upload(
        &config,
        &pool,
        NewUpload {
            file_name: payload.file_name,
            file_type: payload.file_type,
            post_id: Some(post_id),
        },
    )
    .await?;
    Ok(HttpResponse::Created().json(upload))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(get_drafts));
    cfg.route("", web::post().to(create_draft));
    cfg.route("/{id}", web::get().to(get_draft_by_id));
    cfg.route("/{id}", web::put().to(update_draft_by_id));
    cfg.route("/{id}/uploads", web::get().to(get_draft_uploads));
    cfg.route("/{id}/uploads", web::post().to(create_draft_upload));
}
