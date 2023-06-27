use std::{collections::HashMap, env, str::FromStr};

use crate::errors;

#[derive(Clone)]
pub enum Environment {
    DEVELOPMENT,
    PRODUCTION,
}

impl FromStr for Environment {
    type Err = errors::Error;
    fn from_str(s: &str) -> Result<Self, errors::Error> {
        match s {
            "development" => Ok(Environment::DEVELOPMENT),
            "production" => Ok(Environment::PRODUCTION),
            _ => Err(errors::Error::InvalidEnv(String::from("ENV"))),
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
    pub smtp_user: String,
    pub smtp_password: String,
}

impl Config {
    pub fn from_env(vars: env::Vars) -> Result<Config, errors::Error> {
        let map: HashMap<String, String> = HashMap::from_iter(vars);

        return Ok(Config {
            host: map
                .get("HOST")
                .ok_or(errors::Error::MissingEnv(format!("HOST")))?
                .to_string(),
            port: map
                .get("PORT")
                .ok_or(errors::Error::MissingEnv(format!("PORT")))?
                .parse()
                .map_err(|_| errors::Error::InvalidEnv(format!("PORT")))?,
            database_url: map
                .get("DATABASE_URL")
                .ok_or(errors::Error::MissingEnv(format!("DATABASE_URL")))?
                .to_string(),
            environment: map
                .get("ENVIRONMENT")
                .unwrap_or(&String::from("production"))
                .parse()?,
            session_key: map
                .get("SESSION_KEY")
                .ok_or(errors::Error::MissingEnv(format!("SESSION_KEY")))?
                .to_string(),
            from_email: map
                .get("FROM_EMAIL")
                .ok_or(errors::Error::MissingEnv(format!("FROM_EMAIL")))?
                .to_string(),
            from_name: map
                .get("FROM_NAME")
                .ok_or(errors::Error::MissingEnv(format!("FROM_NAME")))?
                .to_string(),
            smtp_host: map
                .get("SMTP_HOST")
                .ok_or(errors::Error::MissingEnv(format!("SMTP_HOST")))?
                .to_string(),
            smtp_user: map
                .get("SMTP_USER")
                .ok_or(errors::Error::MissingEnv(format!("SMTP_USER")))?
                .to_string(),
            smtp_password: map
                .get("SMTP_PASSWORD")
                .ok_or(errors::Error::MissingEnv(format!("SMTP_PASSWORD")))?
                .to_string(),
        });
    }
}
