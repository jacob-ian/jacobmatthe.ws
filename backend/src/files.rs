use crate::{config::Config, db::uploads::Upload, errors::Error};
use std::{fs::File, io::Write, path::Path};

pub fn create_file_from_upload(
    config: &Config,
    upload: &Upload,
    buffer: &[u8],
) -> Result<(), Error> {
    let path = format!("{}/{}", config.uploads.dir, &upload.file_name);
    let exists = File::open(&path).is_ok();
    if exists {
        return Err(Error::BadRequestError(format!("File already exists")));
    }
    File::create(&path)
        .map_err(|_| Error::InternalServerError(format!("Could not create file")))?
        .write(buffer)
        .map_err(|_| Error::InternalServerError(format!("Could not write to file")))?;
    Ok(())
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
