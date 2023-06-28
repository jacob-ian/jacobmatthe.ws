use std::{collections::HashMap, env, fs, path::Path};

use serde::Deserialize;

use crate::{errors::Error, files};

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
    pub whitelist: Option<Vec<String>>,
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
            .map_err(|_| Error::ConfigError(format!("Uploads directory does not exist")))?;
        Ok(())
    }
}
