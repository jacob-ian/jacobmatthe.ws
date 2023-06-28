use std::{collections::HashMap, env, fs, path::Path, str::FromStr};

use serde::Deserialize;

use crate::{errors::Error, files};

#[derive(Clone, Deserialize)]
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

#[derive(Deserialize, Clone)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_password: String,
    pub sender_name: String,
    pub sender_address: String,
}

#[derive(Deserialize, Clone)]
pub struct AuthConfig {
    pub session_key: String,
    pub secure_cookies: bool,
}

#[derive(Deserialize, Clone)]
pub struct UploadsConfig {
    pub dir: String,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub auth: AuthConfig,
    pub email: EmailConfig,
    pub uploads: UploadsConfig,
}

impl Config {
    pub fn from_toml() -> Result<Config, Error> {
        let path = Self::find_config_path()
            .ok_or(Error::NotFoundError(format!("Could not find config file")))?;
        println!("Config file found: {}", path);
        let config_str =
            fs::read_to_string(&path).map_err(|e| Error::ConfigError(e.to_string()))?;
        let config: Config =
            toml::from_str(&config_str).map_err(|e| Error::ConfigError(e.to_string()))?;
        Self::verify_config(&config)?;
        Ok(config)
    }

    fn find_config_path() -> Option<String> {
        let locations: Vec<String> = vec![
            String::from("./config.toml"),
            String::from("/etc/jacobmatthe.ws/config.toml"),
        ];
        let mut i = 0;
        loop {
            if i == locations.len() {
                break None;
            }
            let location = locations.get(i).expect("Config path index out of bounds");
            if Path::new(&location).is_file() {
                break Some(location.to_owned());
            }
            i = i + 1;
        }
    }

    fn verify_config(config: &Config) -> Result<(), Error> {
        files::check_directory(&config.uploads.dir)
            .map_err(|e| Error::ConfigError(format!("Uploads directory does not exist")))
    }

    pub fn from_env(vars: env::Vars) -> Result<Config, Error> {
        let map: HashMap<String, String> = HashMap::from_iter(vars);
        let email_config = EmailConfig {
            sender_address: map
                .get("SENDER_ADDRESS")
                .ok_or(Error::MissingEnv(format!("SENDER_ADDRESS")))?
                .to_string(),
            sender_name: map
                .get("SENDER_NAME")
                .ok_or(Error::MissingEnv(format!("SENDER_NAME")))?
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
        };
        let auth_config = AuthConfig {
            session_key: map
                .get("SESSION_KEY")
                .ok_or(Error::MissingEnv(format!("SESSION_KEY")))?
                .to_string(),
            secure_cookies: map
                .get("ENVIRONMENT")
                .unwrap_or(&String::from("production"))
                .to_string()
                == format!("production"),
        };
        let uploads_config = UploadsConfig {
            dir: map
                .get("UPLOADS_DIR")
                .ok_or(Error::MissingEnv(format!("UPLOADS_DIR")))?
                .to_string(),
        };
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
            email: email_config,
            auth: auth_config,
            uploads: uploads_config,
        };
        Self::verify_config(&config);
        Ok(config)
    }
}
