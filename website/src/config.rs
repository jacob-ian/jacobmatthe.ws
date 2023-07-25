use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

use serde::Deserialize;
use toml;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub backend_uri: String,
}

impl Config {
    pub fn from_toml() -> Result<Config, Error> {
        let path = Self::find_config_path().ok_or(Error::new(
            ErrorKind::NotFound,
            "Could not find config file",
        ))?;
        println!("Config file found: {}", path);
        let config_str = fs::read_to_string(&path)?;
        let config: Config =
            toml::from_str(&config_str).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        Ok(config)
    }

    fn find_config_path() -> Option<String> {
        let locations: Vec<String> = vec![
            String::from("./config.toml"),
            String::from("/etc/jacobmatthe.ws/website.config.toml"),
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
}
