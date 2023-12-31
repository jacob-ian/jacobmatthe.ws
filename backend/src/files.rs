use sqlx::PgPool;

use crate::{
    config::Config,
    db::{
        self,
        uploads::{NewUpload, Upload},
    },
    errors::Error,
};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub fn upload_file(config: &Config, upload: &Upload, buffer: &[u8]) -> Result<(), Error> {
    let path = create_path(&config.uploads.dir, &upload.file_name);
    let exists = Path::new(&path).is_file();
    if exists {
        return Err(Error::BadRequestError(format!("File already exists")));
    }
    File::create(&path)
        .map_err(|_| Error::InternalServerError(format!("Could not create file")))?
        .write(buffer)
        .map_err(|_| Error::InternalServerError(format!("Could not write to file")))?;
    Ok(())
}

fn create_path(dir: &String, file_name: &String) -> String {
    format!("{}/{}", dir, file_name)
}

pub async fn create_file_upload(
    config: &Config,
    pool: &PgPool,
    new_upload: NewUpload,
) -> Result<Upload, Error> {
    if file_exists(config, &new_upload.file_name) {
        return Err(Error::BadRequestError(format!(
            "A file with that name already exists"
        )));
    }
    db::uploads::create_upload(pool, new_upload).await
}

pub async fn delete_file_upload(
    config: &Config,
    pool: &PgPool,
    upload: &Upload,
) -> Result<(), Error> {
    let path = create_path(&config.uploads.dir, &upload.file_name);
    fs::remove_file(&path).map_err(|e| {
        Error::InternalServerError(format!("Couldn't delete upload: {}", e.to_string()))
    })?;
    db::uploads::delete_by_id(pool, upload.id).await
}

pub fn check_directory(directory: &String) -> Result<(), Error> {
    let exists: bool = Path::new(directory).is_dir();
    if !exists {
        return Err(Error::NotFoundError(format!("Directory does not exist")));
    }
    Ok(())
}

pub fn file_exists(config: &Config, file_name: &str) -> bool {
    let path = format!("{}/{}", config.uploads.dir, file_name);
    Path::new(&path).is_file()
}
