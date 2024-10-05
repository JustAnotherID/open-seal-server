use anyhow::Error;
use log::info;
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::io::Write;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Config {
    pub(crate) server: ServerConfig,
    pub(crate) database: DbConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct ServerConfig {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) max_log_mb: usize,
    pub(crate) domain: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub(crate) enum DbConfig {
    #[serde(rename = "sqlite")]
    Sqlite { path: String },

    #[serde(rename = "postgres")]
    Postgres { url: String },
}

pub(crate) fn read_config() -> Result<Config, Error> {
    if let Ok(config) = read_to_string("config.toml") {
        info!("The config.toml found, using it");
        let conf: Config = toml::from_str(config.as_str()).expect(
            "Failed to read config.toml. Check the file or delete it to generate a new one",
        );
        Ok(conf)
    } else {
        // info!("config.toml not found. starting interactive setup...");
        info!("The config.toml not found. Generating default config...");
        setup()
    }
}

fn setup() -> Result<Config, Error> {
    let mut file = File::create("config.toml")?;
    let conf = Config {
        server: ServerConfig {
            host: "0.0.0.0".to_string(),
            port: 3212,
            max_log_mb: 10,
            domain: "http://localhost:3212".to_string(),
        },
        database: DbConfig::Sqlite {
            path: "data.db".to_string(),
        },
    }; // TODO interactive setup
    let data = toml::to_string(&conf)?;
    file.write_all(data.as_bytes())?;
    Ok(conf)
}
