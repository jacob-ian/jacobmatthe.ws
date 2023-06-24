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

pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub environment: Environment,
    pub session_key: String,
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
            redis_url: map
                .get("REDIS_URL")
                .ok_or(errors::Error::MissingEnv(format!("REDIS_URL")))?
                .to_string(),
            environment: map
                .get("ENVIRONMENT")
                .unwrap_or(&String::from("production"))
                .parse()?,
            session_key: map
                .get("SESSION_KEY")
                .ok_or(errors::Error::MissingEnv(format!("SESSION_KEY")))?
                .to_string(),
        });
    }
}
