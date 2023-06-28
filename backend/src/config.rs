use std::{collections::HashMap, env, str::FromStr};

use crate::{errors::Error, files};

#[derive(Clone)]
pub enum Environment {
    DEVELOPMENT,
    PRODUCTION,
}

impl FromStr for Environment {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "development" => Ok(Environment::DEVELOPMENT),
            "production" => Ok(Environment::PRODUCTION),
            _ => Err(Error::InvalidEnv(String::from("ENV"))),
        }
    }
}

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub environment: Environment,
    pub session_key: String,
    pub from_email: String,
    pub from_name: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_password: String,
    pub uploads_dir: String,
}

impl Config {
    pub fn from_env(vars: env::Vars) -> Result<Config, Error> {
        let map: HashMap<String, String> = HashMap::from_iter(vars);
        let config = Config {
            host: map
                .get("HOST")
                .ok_or(Error::MissingEnv(format!("HOST")))?
                .to_string(),
            port: map
                .get("PORT")
                .ok_or(Error::MissingEnv(format!("PORT")))?
                .parse()
                .map_err(|_| Error::InvalidEnv(format!("PORT")))?,
            database_url: map
                .get("DATABASE_URL")
                .ok_or(Error::MissingEnv(format!("DATABASE_URL")))?
                .to_string(),
            environment: map
                .get("ENVIRONMENT")
                .unwrap_or(&String::from("production"))
                .parse()?,
            session_key: map
                .get("SESSION_KEY")
                .ok_or(Error::MissingEnv(format!("SESSION_KEY")))?
                .to_string(),
            from_email: map
                .get("FROM_EMAIL")
                .ok_or(Error::MissingEnv(format!("FROM_EMAIL")))?
                .to_string(),
            from_name: map
                .get("FROM_NAME")
                .ok_or(Error::MissingEnv(format!("FROM_NAME")))?
                .to_string(),
            smtp_host: map
                .get("SMTP_HOST")
                .ok_or(Error::MissingEnv(format!("SMTP_HOST")))?
                .to_string(),
            smtp_port: map
                .get("SMTP_PORT")
                .ok_or(Error::MissingEnv(format!("SMTP_PORT")))?
                .parse()
                .map_err(|_| Error::InvalidEnv(format!("SMTP_PORT")))?,
            smtp_user: map
                .get("SMTP_USER")
                .ok_or(Error::MissingEnv(format!("SMTP_USER")))?
                .to_string(),
            smtp_password: map
                .get("SMTP_PASSWORD")
                .ok_or(Error::MissingEnv(format!("SMTP_PASSWORD")))?
                .to_string(),
            uploads_dir: map
                .get("UPLOADS_DIR")
                .ok_or(Error::MissingEnv(format!("UPLOADS_DIR")))?
                .to_string(),
        };
        files::check_directory(&config.uploads_dir)
            .map_err(|_| Error::InvalidEnv(format!("UPLOADS_DIR does not exist")))?;

        Ok(config)
    }
}
