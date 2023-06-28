use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use futures_util::StreamExt;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::Config,
    db,
    errors::Error,
    files::{self, file_exists},
};

#[derive(Deserialize)]
struct CreateUploadBody {
    file_name: String,
}

async fn create_upload(
    session: Session,
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    body: web::Json<CreateUploadBody>,
) -> Result<HttpResponse, Error> {
    if let Ok(None) = session.get::<Uuid>("user_id") {
        return Err(Error::UnauthorizedError(format!("Unauthorized")));
    }
    let payload = body.into_inner();
    if file_exists(&config, &payload.file_name) {
        return Err(Error::BadRequestError(format!(
            "File with that name already exists"
        )));
    }
    let upload = db::uploads::create_upload(&pool, payload.file_name).await?;
    Ok(HttpResponse::Ok().json(upload))
}

async fn upload_file(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    session: Session,
    path: web::Path<Uuid>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    if let Ok(None) = session.get::<Uuid>("user_id") {
        return Err(Error::UnauthorizedError(format!("Unauthorized")));
    }
    let upload_id = path.into_inner();
    let upload = db::uploads::get_incoming_upload_by_id(&pool, upload_id).await?;
    let mut buffer: Vec<u8> = Vec::new();
    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|_| Error::InternalServerError(format!("Upload failed")))?;
        while let Some(chunk) = field.next().await {
            let bytes = chunk
                .map_err(|_| Error::InternalServerError(format!("Caught bad chunk")))?
                .to_vec();
            for byte in bytes {
                buffer.push(byte);
            }
        }
    }
    files::create_file_from_upload(&config, &upload, &buffer)?;
    db::uploads::set_upload_complete_by_id(&pool, upload_id).await?;
    Ok(HttpResponse::Ok().into())
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("").route(web::post().to(create_upload)));
    cfg.service(web::resource("/{upload_id}").route(web::put().to(upload_file)));
}